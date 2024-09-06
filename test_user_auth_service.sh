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
    
    echo "==== Request Details ===="
    echo "Method: $method"
    echo "Endpoint: $endpoint"
    echo "Data: $data"
    echo "Token: $token"
    echo "Expected Status: $expected_status"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    headers=(-H "Accept: application/json")
    if [ -n "$data" ]; then
        headers+=(-H "Content-Type: application/json")
    fi
    if [ -n "$token" ]; then
        headers+=(-H "Authorization: Bearer $token")
    fi
    
    echo "==== Request Headers ===="
    printf '%s\n' "${headers[@]}"

    echo "==== Curl Command ===="
    if [ -n "$data" ]; then
        echo "curl -v -X $method ${headers[@]} -d '$data' $BASE_URL$endpoint"
        response=$(curl -v -X $method "${headers[@]}" -d "$data" "$BASE_URL$endpoint" 2>&1)
    else
        echo "curl -v -X $method ${headers[@]} $BASE_URL$endpoint"
        response=$(curl -v -X $method "${headers[@]}" "$BASE_URL$endpoint" 2>&1)
    fi
    
    echo "==== Response ===="
    echo "$response"
    
    status_code=$(echo "$response" | grep -i "< HTTP" | awk '{print $3}')
    body=$(echo "$response" | sed -n '/^{/,/^}/p')
    
    echo "==== Status Code ===="
    echo "$status_code"
    
    if [ "$status_code" = "$expected_status" ]; then
        echo -e "\nTest passed: $method $endpoint (Status: $status_code)"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "\nTest failed: $method $endpoint (Expected: $expected_status, Got: $status_code)"
    fi
    
    echo "==== Response Body ===="
    echo "$body"
    echo "======================="
}

# Create a new user
echo "Creating a new user"
response=$(make_request POST "/users" '{"username": "testuser", "email": "testuser@example.com", "password": "testpass"}' "" "201")
user_id=$(echo "$response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "User ID: $user_id"

# Login with the new user
echo "Logging in with the new user"
login_response=$(make_request POST "/users/login" '{"username": "testuser", "password": "testpass"}' "" "200")
token=$(echo "$login_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Token: $token"

# List users
echo -e "\n\n\nListing users"
make_request GET "/users" "" "" "200"

# Get user details
echo -e "\n\n\n Get user Details"
make_request GET "/users/$user_id" "" "$token" "200"

# Update user details
echo "Update User Details"
make_request PUT "/users/$user_id" '{"email": "updated@example.com"}' "$token" "200"

# Verify the update
echo "Verify User Update"
make_request GET "/users/$user_id" "" "$token" "200"

# Attempt to create a user with the same email
echo "Attempt creation of duplicate user"
make_request POST "/users" '{"username": "testuser2", "email": "updated@example.com", "password": "testpass2"}' "" "400"

# Refresh token
echo "Refresh Token"
refresh_response=$(make_request POST "/users/refresh" "" "$token" "200")
new_token=$(echo "$refresh_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "New token: $new_token"

# Use the new token to get user details
echo "Get user with new token"
make_request GET "/users/$user_id" "" "$new_token" "200"

# Delete user
echo -e "\n\n\n delete user"
make_request DELETE "/users/$user_id" "" "$token" "204"

# Verify the deletion
echo "Verify delete user"
make_request GET "/users/$user_id" "" "$token" "500"

# Print summary
echo "\n\nTest Summary:"
echo "Total tests: $TOTAL_TESTS"
echo "Passed tests: $PASSED_TESTS"
echo "Failed tests: $((TOTAL_TESTS - PASSED_TESTS))"
echo "Success rate: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%"

echo "User authentication testing completed."