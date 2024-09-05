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
    
    echo "Testing $method $endpoint"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if [ -n "$data" ]; then
        if [ -n "$token" ]; then
            response=$(curl -s -w "\n%{http_code}" -X $method -H "Content-Type: application/json" -H "Authorization: Bearer $token" -d "$data" $BASE_URL$endpoint)
        else
            response=$(curl -s -w "\n%{http_code}" -X $method -H "Content-Type: application/json" -d "$data" $BASE_URL$endpoint)
        fi
    else
        if [ -n "$token" ]; then
            response=$(curl -s -w "\n%{http_code}" -X $method -H "Authorization: Bearer $token" $BASE_URL$endpoint)
        else
            response=$(curl -s -w "\n%{http_code}" -X $method $BASE_URL$endpoint)
        fi
    fi
    
    status_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')
    
    if [ "$status_code" = "$expected_status" ]; then
        echo "Test passed: $method $endpoint (Status: $status_code)"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "Test failed: $method $endpoint (Expected: $expected_status, Got: $status_code)"
    fi
    
    echo -e "Response body: $body\n"
    echo "$body"
}

# Create a new user
echo "Creating a new user"
response=$(make_request POST "/users" '{"username": "testuser", "email": "testuser@example.com", "password": "testpass"}' "" "201")
user_id=$(echo "$response" | grep -o '"id":"[^"]*' | cut -d'"' -f4)
echo "User ID: $user_id"

# Login with the new user
echo "Logging in with the new user"
login_response=$(make_request POST "/users/login" '{"username": "testuser", "password": "testpass"}' "" "200")
token=$(echo "$login_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4)
echo "Token: $token"

# List users
make_request GET "/users" "" "" "200"

# Get user details
make_request GET "/users/$user_id" "" "$token" "200"

# Update user details
make_request PUT "/users/$user_id" '{"email": "updated@example.com"}' "$token" "200"

# Verify the update
make_request GET "/users/$user_id" "" "$token" "200"

# Attempt to create a user with the same email
make_request POST "/users" '{"username": "testuser2", "email": "updated@example.com", "password": "testpass2"}' "" "400"

# Refresh token
refresh_response=$(make_request POST "/users/refresh" "" "$token" "200")
new_token=$(echo "$refresh_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4)
echo "New token: $new_token"

# Use the new token to get user details
make_request GET "/users/$user_id" "" "$new_token" "200"

# Delete user
make_request DELETE "/users/$user_id" "" "$token" "204"

# Verify the deletion
make_request GET "/users/$user_id" "" "$token" "404"

# Print summary
echo "Test Summary:"
echo "Total tests: $TOTAL_TESTS"
echo "Passed tests: $PASSED_TESTS"
echo "Failed tests: $((TOTAL_TESTS - PASSED_TESTS))"
echo "Success rate: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%"

echo "User authentication testing completed."
