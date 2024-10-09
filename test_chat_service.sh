#!/bin/bash

BASE_URL="http://localhost:8000"
TOTAL_TESTS=0
PASSED_TESTS=0

# Set log level
export RUST_LOG=debug

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
    echo "Full URI: $BASE_URL$endpoint"

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
        response=$(curl -v -s -w "\n%{http_code}" -X $method "${headers[@]}" -d "$data" "$BASE_URL$endpoint")
    else
        response=$(curl -v -s -w "\n%{http_code}" -X $method "${headers[@]}" "$BASE_URL$endpoint")
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

# Generate a unique email for this test run
UNIQUE_EMAIL="testuser_$(date +%s)@example.com"

# Create a new user
echo "Creating a new user"
create_user_response=$(make_request POST "/users" "{\"username\": \"testuser\", \"email\": \"$UNIQUE_EMAIL\", \"password\": \"testpass\"}" "" "201")
user_id=$(echo "$create_user_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "User ID: $user_id"

# Login with the new user
echo "Logging in with the new user"
login_response=$(make_request POST "/users/login" '{"username": "testuser", "password": "testpass"}' "" "200")
token=$(echo "$login_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Token: $token"

if [ -z "$token" ] || [ "$token" == "null" ]; then
    echo "Failed to obtain authentication token. Exiting."
    exit 1
fi

echo "Successfully obtained authentication token."

# Validate token
echo "Validating token"
make_request GET "/users/validate-token" "" "$token" "200"

# Test sending a chat message
echo "Testing send message..."
send_message_response=$(make_request POST "/chat" '{"content":"Hello, AI!"}' "$token" "200")
echo "Send message response: $send_message_response"

# Test chat stream
echo "Testing chat stream..."
stream_response=$(make_request GET "/chat/stream?content=Tell%20me%20a%20joke" "" "$token" "200")
echo "Stream response: $stream_response"

# Delete user
echo "Deleting user"
make_request DELETE "/users/$user_id" "" "$token" "204"

# Print summary
echo -e "\n----- Test Summary -----"
echo "Total tests: $TOTAL_TESTS"
echo "Passed tests: $PASSED_TESTS"
echo "Failed tests: $((TOTAL_TESTS - PASSED_TESTS))"
if [ "$TOTAL_TESTS" -ne 0 ]; then
    echo "Success rate: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%"
else
    echo "No tests were run."
fi

echo -e "\nChat service testing completed."