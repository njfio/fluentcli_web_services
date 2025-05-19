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

# Function to validate user LLM config fields
validate_user_llm_config() {
    local config_data=$1
    local user_id=$2
    local provider_id=$3
    local api_key_id=$4

    echo "Validating user LLM config fields:"
    
    if [[ $config_data == *"\"user_id\":\"$user_id\""* ]]; then
        echo "✓ User ID is correct"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "✗ User ID is incorrect"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if [[ $config_data == *"\"provider_id\":\"$provider_id\""* ]]; then
        echo "✓ Provider ID is correct"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "✗ Provider ID is incorrect"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if [[ $config_data == *"\"api_key_id\":\"$api_key_id\""* ]]; then
        echo "✓ API Key ID is correct"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "✗ API Key ID is incorrect"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
}

# Generate a unique username
TIMESTAMP=$(date +%s)
USERNAME="llmuser_${TIMESTAMP}"

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
    "key_value": "test_api_key_value",
    "description": "Test API Key"
}' "$token" "201")
api_key_id=$(echo "$api_key_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "API Key ID: $api_key_id"

if [ -z "$api_key_id" ]; then
    echo "Failed to create API key. Exiting."
    exit 1
fi

# Create a new LLM provider
echo "Creating a new LLM provider"
provider_response=$(make_request POST "/llm/providers" '{
    "user_id": "'"$user_id"'",
    "name": "TestProvider",
    "provider_type": "openai",
    "api_endpoint": "https://api.openai.com/v1",
    "supported_modalities": ["text"],
    "configuration": {
        "model": "gpt-3.5-turbo",
        "max_tokens": 150
    }
}' "$token" "201")
provider_id=$(echo "$provider_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Provider ID: $provider_id"

if [ -z "$provider_id" ]; then
    echo "Failed to create LLM provider. Exiting."
    exit 1
fi

# Test user LLM config operations
echo "Testing user LLM config operations"

# Create a new user LLM config
echo "Creating a new user LLM config"
config_response=$(make_request POST "/llm/user-configs" '{
    "user_id": "'"$user_id"'",
    "provider_id": "'"$provider_id"'",
    "api_key_id": "'"$api_key_id"'"
}' "$token" "201")
config_id=$(echo "$config_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Config ID: $config_id"

if [ -z "$config_id" ]; then
    echo "Failed to create user LLM config. Exiting."
    exit 1
fi

# Validate the created user LLM config
validate_user_llm_config "$config_response" "$user_id" "$provider_id" "$api_key_id"

# Get the created user LLM config
echo "Getting the created user LLM config"
get_config_response=$(make_request GET "/llm/user-configs/$config_id" "" "$token" "200")

# Validate the retrieved user LLM config
validate_user_llm_config "$get_config_response" "$user_id" "$provider_id" "$api_key_id"

# List all user LLM configs
echo "Listing all user LLM configs"
make_request GET "/llm/user-configs" "" "$token" "200"

# Update the user LLM config
echo "Updating the user LLM config"
update_config_response=$(make_request PUT "/llm/user-configs/$config_id" '{
    "user_id": "'"$user_id"'",
    "provider_id": "'"$provider_id"'",
    "api_key_id": "'"$api_key_id"'"
}' "$token" "200")

# Validate the updated user LLM config
validate_user_llm_config "$update_config_response" "$user_id" "$provider_id" "$api_key_id"

# Delete the user LLM config
echo "Deleting the user LLM config"
make_request DELETE "/llm/user-configs/$config_id" "" "$token" "204"

# Verify the user LLM config was deleted
echo "Verifying the user LLM config was deleted"
make_request GET "/llm/user-configs/$config_id" "" "$token" "404"

# Test error cases
echo "Testing error cases"

# Test creating a user LLM config with an invalid provider ID
echo "Testing creation of user LLM config with invalid provider ID"
make_request POST "/llm/user-configs" '{
    "user_id": "'"$user_id"'",
    "provider_id": "00000000-0000-0000-0000-000000000000",
    "api_key_id": "'"$api_key_id"'"
}' "$token" "400"

# Test getting a non-existent user LLM config
echo "Testing retrieval of non-existent user LLM config"
make_request GET "/llm/user-configs/00000000-0000-0000-0000-000000000000" "" "$token" "404"

# Clean up
echo "Cleaning up"

# Delete the LLM provider
echo "Deleting the LLM provider"
make_request DELETE "/llm/providers/$provider_id" "" "$token" "204"

# Delete the API key
echo "Deleting the API key"
make_request DELETE "/api_keys/$api_key_id" "" "$token" "204"

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
