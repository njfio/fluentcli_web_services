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

# Function to create and test an API key
create_and_test_api_key() {
    local name=$1
    local api_key
    
    case $name in
        "OpenAI")
            api_key="$AMBER_FLUENT_OPENAI_API_KEY"
            ;;
        "Anthropic")
            api_key="$AMBER_FLUENT_ANTHROPIC_KEY_01"
            ;;
        "Cohere")
            api_key="$AMBER_FLUENT_COHERE_API_KEY_01"
            ;;
        *)
            echo "Unknown provider $name. Skipping."
            return
            ;;
    esac

    if [ -z "$api_key" ]; then
        echo "API key for $name is not set. Skipping."
        return
    fi

    echo "Creating a new API key for $name"
    api_key_response=$(make_request POST "/api_keys" '{
        "key_value": "'"$api_key"'",
        "description": "Test '"$name"' API Key"
    }' "$token" "201")
    api_key_id=$(echo "$api_key_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
    returned_key_value=$(echo "$api_key_response" | grep -o '"key_value":"[^"]*' | cut -d'"' -f4 | head -n 1)
    echo "$name API Key ID: $api_key_id"
    echo "$name API Key Value: $returned_key_value"

    if [ -z "$api_key_id" ]; then
        echo "Failed to create $name API key. Skipping further tests."
        return
    fi

    # Validate that the API key is not encrypted in the response
    if [ "$returned_key_value" = "$api_key" ]; then
        echo -e "\e[32m✓ PASS\e[0m $name API key is not encrypted in the creation response"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "\e[31m✗ FAIL\e[0m $name API key appears to be encrypted in the creation response"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    # Get the created API key
    echo "Getting the created $name API key"
    get_key_response=$(make_request GET "/api_keys/$api_key_id" "" "$token" "200")
    retrieved_key_value=$(echo "$get_key_response" | grep -o '"key_value":"[^"]*' | cut -d'"' -f4 | head -n 1)

    # Validate that the retrieved API key is not encrypted
    if [ "$retrieved_key_value" = "$api_key" ]; then
        echo -e "\e[32m✓ PASS\e[0m Retrieved $name API key is not encrypted"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "\e[31m✗ FAIL\e[0m Retrieved $name API key appears to be encrypted"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    # Update the API key
    echo "Updating the $name API key"
    make_request PUT "/api_keys/$api_key_id" '{
        "description": "Updated '"$name"' Test API Key"
    }' "$token" "200"

    # Get the updated API key
    echo "Getting the updated $name API key"
    updated_key_response=$(make_request GET "/api_keys/$api_key_id" "" "$token" "200")
    updated_key_value=$(echo "$updated_key_response" | grep -o '"key_value":"[^"]*' | cut -d'"' -f4 | head -n 1)

    # Validate that the updated API key is not encrypted
    if [ "$updated_key_value" = "$api_key" ]; then
        echo -e "\e[32m✓ PASS\e[0m Updated $name API key is not encrypted"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "\e[31m✗ FAIL\e[0m Updated $name API key appears to be encrypted"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    # Delete the API key
    echo "Deleting the $name API key"
    make_request DELETE "/api_keys/$api_key_id" "" "$token" "204"

    # Attempt to get the deleted API key (should fail)
    echo "Attempting to get the deleted $name API key"
    make_request GET "/api_keys/$api_key_id" "" "$token" "404"
}

# Create and test API keys for each provider
create_and_test_api_key "OpenAI"
create_and_test_api_key "Anthropic"
create_and_test_api_key "Cohere"

# List all API keys for the user
echo "Listing all API keys for the user"
list_keys_response=$(make_request GET "/api_keys" "" "$token" "200")

# Validate that the listed API keys are not encrypted
echo "$list_keys_response" | jq -r '.[] | .key_value' | while read -r listed_key_value; do
    if [[ "$listed_key_value" == "$AMBER_FLUENT_OPENAI_API_KEY" || 
          "$listed_key_value" == "$AMBER_FLUENT_ANTHROPIC_KEY_01" || 
          "$listed_key_value" == "$AMBER_FLUENT_COHERE_API_KEY_01" ]]; then
        echo -e "\e[32m✓ PASS\e[0m Listed API key is not encrypted"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "\e[31m✗ FAIL\e[0m Listed API key appears to be encrypted"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
done

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
