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
        response=$(curl -s -w "\n%{http_code}" -X $method "${headers[@]}" -d "$data" "$BASE_URL$endpoint")
    else
        response=$(curl -s -w "\n%{http_code}" -X $method "${headers[@]}" "$BASE_URL$endpoint")
    fi
    
    status_code=$(echo "$response" | tail -n 1)
    body=$(echo "$response" | sed '$d')
    
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
    echo "$body"
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

# Create a new configuration entry
echo "Creating a new configuration entry"
response=$(make_request POST "/configurations" '{"name": "test_config", "data": {"key": "value"}}' "$token" "201")
configuration_id=$(echo "$response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Configuration ID: $configuration_id"

# List configuration entries
echo -e "\n\n\nListing configuration entries"
make_request GET "/configurations" "" "$token" "200"

# Get configuration entry details
echo -e "\n\n\nGet configuration entry details"
make_request GET "/configurations/$configuration_id" "" "$token" "200"

# Update configuration entry
echo -e "\n\n\nUpdate configuration entry"
make_request PUT "/configurations/$configuration_id" '{"name": "updated_config", "data": {"key": "updated_value"}}' "$token" "200"

# Verify the update
echo -e "\n\n\nVerify configuration entry update"
make_request GET "/configurations/$configuration_id" "" "$token" "200"

# Delete configuration entry
echo -e "\n\n\nDelete configuration entry"
make_request DELETE "/configurations/$configuration_id" "" "$token" "204"

# Verify the deletion
echo -e "\n\n\nVerify configuration entry deletion"
make_request GET "/configurations/$configuration_id" "" "$token" "500"

# Delete the user
echo -e "\n\n\nDeleting the user"
make_request DELETE "/users/$user_id" "" "$token" "204"

# Print summary
echo "\n\nTest Summary:"
echo "Total tests: $TOTAL_TESTS"
echo "Passed tests: $PASSED_TESTS"
echo "Failed tests: $((TOTAL_TESTS - PASSED_TESTS))"
echo "Success rate: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%"

echo "Configuration testing completed."