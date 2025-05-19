#!/bin/bash

BASE_URL="http://localhost:8000"
TOTAL_TESTS=0
PASSED_TESTS=0

# Function to make API requests
make_request() {
    local method=$1
    local endpoint=$2
    local data=$3
    local token=$4
    local expected_status=$5
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    headers=(-H "Accept: application/json")
    if [ -n "$data" ]; then
        headers+=(-H "Content-Type: application/json")
    fi
    if [ -n "$token" ]; then
        headers+=(-H "Authorization: Bearer $token")
    fi

    echo -e "Request details:"
    echo "Method: $method"
    echo "Endpoint: $endpoint"
    echo "Data: $data"
    echo "Token: $token"
    echo "Expected status: $expected_status"

    if [ -n "$data" ]; then
        response=$(curl -v -s -w "\n%{http_code}" -X $method "${headers[@]}" -d "$data" "$BASE_URL$endpoint" 2>&1)
    else
        response=$(curl -v -s -w "\n%{http_code}" -X $method "${headers[@]}" "$BASE_URL$endpoint" 2>&1)
    fi
    
    status_code=$(echo "$response" | tail -n 1)
    body=$(echo "$response" | sed '$d' | sed -n '/^{/,/^}/p' | tr -d '\n')
    
    echo "Response details:"
    echo "Status code: $status_code"
    echo "Body: $body"
    
    if [ "$status_code" = "$expected_status" ]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        print_result "PASS" "$method" "$endpoint" "$expected_status" "$status_code"
    else
        print_result "FAIL" "$method" "$endpoint" "$expected_status" "$status_code"
    fi
    
    #echo "$body"
}

# Function to print test results
print_result() {
    local result=$1
    local method=$2
    local endpoint=$3
    local expected=$4
    local actual=$5
    
    if [ "$result" = "PASS" ]; then
        echo -e "${GREEN}✓ PASS${NC} $method $endpoint (Expected: $expected, Got: $actual)\n"
    else
        echo -e "${RED}✗ FAIL${NC} $method $endpoint (Expected: $expected, Got: $actual)\n"
    fi
}

# Create a new user
echo "Creating a new user"
response=$(make_request POST "/users" '{"username": "testuser", "email": "chatuser@example.com", "password": "testpass"}' "" "201")
user_id=$(echo "$response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "User ID: $user_id"

# Login with the new user
echo "Logging in with the new user"
login_response=$(make_request POST "/users/login" '{"username": "testuser", "password": "testpass"}' "" "200")
token=$(echo "$login_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Token: $token"

# Validate token
echo "Validating token"
echo "Token being sent: $token"
make_request GET "/users/validate-token" "" "$token" "200"

# List users
echo "Listing users"
make_request GET "/users" "" "$token" "200"

# Get user details
echo "Get user details"
make_request GET "/users/$user_id" "" "$token" "200"

# Update user details
echo "Update user details"
make_request PUT "/users/$user_id" '{"email": "updated@example.com"}' "$token" "200"

# Verify the update
echo "Verify user update"
make_request GET "/users/$user_id" "" "$token" "200"

# Attempt to create a user with the same email
echo "Attempt creation of duplicate user"
make_request POST "/users" '{"username": "testuser2", "email": "updated@example.com", "password": "testpass2"}' "" "400"

# Refresh token
echo "Refresh token"
refresh_response=$(make_request POST "/users/refresh" "" "$token" "200")
echo "Refresh response: $refresh_response"
new_token=$(echo "$refresh_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "New token: $new_token"

echo "Old token: $token"
echo "New token: $new_token"
if [ "$token" = "$new_token" ]; then
    echo "Error: New token is the same as the old token"
    exit 1
fi

# Use the new token to get user details
echo "Get user with new token"
make_request GET "/users/$user_id" "" "$new_token" "200"

# Validate new token
echo "Validating new token"
echo "New token: $new_token"
make_request GET "/users/validate-token" "" "$new_token" "200"

# Try to use the old token (should fail)
echo "Trying to use old token"
echo "Old token: $token"
echo "New token: $new_token"
make_request GET "/users/$user_id" "" "$token" "404"

# Delete user
#echo "Delete user"
#make_request DELETE "/users/$user_id" "" "$new_token" "204"

# Verify the deletion
#echo "Verify user deletion"
#make_request GET "/users/$user_id" "" "$new_token" "500"

# Try to use the token after user deletion (should fail)
echo "Trying to use token after user deletion"
make_request GET "/users" "" "$new_token" "401"

# Print summary
echo -e "\n----- Test Summary -----"
echo "Total tests: $TOTAL_TESTS"
echo -e "Passed tests: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed tests: ${RED}$((TOTAL_TESTS - PASSED_TESTS))${NC}"
echo -e "Success rate: ${GREEN}$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%${NC}"

echo -e "\nUser authentication testing completed."