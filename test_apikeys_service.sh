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

    echo -e "\n==== Request details ===="
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
    
    echo -e "\n==== Full Response ===="
    echo "$response"
    
    status_code=$(echo "$response" | tail -n 1)
    body=$(echo "$response" | sed '$d' | sed -n '/^{/,/^}/p' | tr -d '\n')
    
    echo -e "\n==== Parsed Response ===="
    echo "Status code: $status_code"
    echo "Body: $body"
    
    if [ "$status_code" = "$expected_status" ]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        print_result "PASS" "$method" "$endpoint" "$expected_status" "$status_code"
    else
        print_result "FAIL" "$method" "$endpoint" "$expected_status" "$status_code"
    fi
    
    echo "$body"
}

# Function to print test results
print_result() {
    local result=$1
    local method=$2
    local endpoint=$3
    local expected=$4
    local actual=$5
    
    if [ "$result" = "PASS" ]; then
        echo -e "\e[32m✓ PASS\e[0m $method $endpoint (Expected: $expected, Got: $actual)\n"
    else
        echo -e "\e[31m✗ FAIL\e[0m $method $endpoint (Expected: $expected, Got: $actual)\n"
    fi
}

# Generate a unique username
TIMESTAMP=$(date +%s)
USERNAME="apiuser_${TIMESTAMP}"

# Create a new user
echo "Creating a new user"
user_response=$(make_request POST "/users" '{"username": "'"$USERNAME"'", "email": "'"$USERNAME@example.com"'", "password": "testpass123"}' "" "201")
user_id=$(echo "$user_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "User ID: $user_id"

if [ -z "$user_id" ]; then
    echo "Failed to create user. Exiting."
    exit 1
fi

# Login with the new user
echo "Logging in with the new user"
login_response=$(make_request POST "/users/login" '{"username": "'"$USERNAME"'", "password": "testpass123"}' "" "200")
token=$(echo "$login_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Token: $token"

if [ -z "$token" ]; then
    echo "Failed to login. Exiting."
    exit 1
fi

# Create a new API key
echo "Creating a new API key"
api_key_response=$(make_request POST "/api_keys" '{
    "description": "Test API Key"
}' "$token" "201")
api_key_id=$(echo "$api_key_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "API Key ID: $api_key_id"

if [ -z "$api_key_id" ]; then
    echo "Failed to create API key. Exiting."
    exit 1
fi

# Get the created API key
echo "Getting the created API key"
make_request GET "/api_keys/$api_key_id" "" "$token" "200"

# List all API keys for the user
echo "Listing all API keys for the user"
make_request GET "/api_keys" "" "$token" "200"

# Update the API key
echo "Updating the API key"
make_request PUT "/api_keys/$api_key_id" '{
    "description": "Updated Test API Key"
}' "$token" "200"

# Get the updated API key
echo "Getting the updated API key"
make_request GET "/api_keys/$api_key_id" "" "$token" "200"

# Delete the API key
#echo "Deleting the API key"
#make_request DELETE "/api_keys/$api_key_id" "" "$token" "204"

# Attempt to get the deleted API key (should fail)
#echo "Attempting to get the deleted API key"
#make_request GET "/api_keys/$api_key_id" "" "$token" "404"

# Print test summary
echo "Total tests: $TOTAL_TESTS"
echo "Passed tests: $PASSED_TESTS"
echo "Failed tests: $((TOTAL_TESTS - PASSED_TESTS))"

if [ $PASSED_TESTS -eq $TOTAL_TESTS ]; then
    echo -e "\e[32mAll tests passed!\e[0m"
    exit 0
else
    echo -e "\e[31mSome tests failed.\e[0m"
    exit 1
fi
