#!/bin/bash

BASE_URL="http://localhost:8000"
TOTAL_TESTS=0
PASSED_TESTS=0

# Check and print environment variables
check_env_var() {
    local var_name=$1
    local var_value=${!var_name}
    if [ -z "$var_value" ]; then
        echo "$var_name is not set"
    else
        echo "$var_name: ${var_value:0:5}..."
    fi
}

echo "Checking environment variables:"
check_env_var "AMBER_FLUENT_OPENAI_API_KEY"
check_env_var "AMBER_FLUENT_ANTHROPIC_KEY_01"
check_env_var "AMBER_FLUENT_COHERE_API_KEY_01"

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

# Function to validate LLM provider fields
validate_llm_provider() {
    local provider_data=$1
    local name=$2
    local provider_type=$3
    local api_endpoint=$4
    local model=$5

    echo "Validating LLM provider fields:"
    
    if [[ $provider_data == *"\"name\":\"$name\""* ]]; then
        echo "✓ Name is correct"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "✗ Name is incorrect"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if [[ $provider_data == *"\"provider_type\":\"$provider_type\""* ]]; then
        echo "✓ Provider type is correct"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "✗ Provider type is incorrect"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if [[ $provider_data == *"\"api_endpoint\":\"$api_endpoint\""* ]]; then
        echo "✓ API endpoint is correct"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "✗ API endpoint is incorrect"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if [[ $provider_data == *"\"supported_modalities\":[\"text\"]"* ]]; then
        echo "✓ Supported modalities are correct"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "✗ Supported modalities are incorrect"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if [[ $provider_data == *"\"model\":\"$model\""* ]]; then
        echo "✓ Model is correct"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "✗ Model is incorrect"
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if [[ $provider_data == *"\"max_tokens\":150"* ]]; then
        echo "✓ Max tokens is correct"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "✗ Max tokens is incorrect"
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

# Function to test a specific provider
test_provider() {
    local name=$1
    local api_key
    local provider_type
    local model
    
    case $name in
        "OpenAI")
            api_key="$AMBER_FLUENT_OPENAI_API_KEY"
            provider_type="openai"
            model="gpt-3.5-turbo"
            ;;
        "Anthropic")
            api_key="$AMBER_FLUENT_ANTHROPIC_KEY_01"
            provider_type="anthropic"
            model="claude-2"
            ;;
        "Cohere")
            api_key="$AMBER_FLUENT_COHERE_API_KEY_01"
            provider_type="cohere"
            model="command"
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

    echo "Testing $name provider"
    # Create a new API key
    echo "Creating a new API key for $name"
    api_key_response=$(make_request POST "/api_keys" '{
        "key_value": "'"$api_key"'",
        "description": "Test '"$name"' API Key"
    }' "$token" "201")
    api_key_id=$(echo "$api_key_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
    echo "$name API Key ID: $api_key_id"

    if [ -z "$api_key_id" ]; then
        echo "Failed to create $name API key. Skipping further tests."
        return
    fi

    # Create a new LLM provider
    echo "Creating a new LLM provider for $name"
    provider_response=$(make_request POST "/llm/providers" '{
        "user_id": "'"$user_id"'",
        "name": "Test'"$name"'Provider",
        "provider_type": "'"$provider_type"'",
        "api_endpoint": "https://api.'"$provider_type"'.com/v1",
        "supported_modalities": ["text"],
        "configuration": {
            "model": "'"$model"'",
            "max_tokens": 150
        }
    }' "$token" "201")
    provider_id=$(echo "$provider_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
    echo "$name Provider ID: $provider_id"

    if [ -z "$provider_id" ]; then
        echo "Failed to create $name LLM provider. Skipping further tests."
        return
    fi

    # Validate the created LLM provider
    validate_llm_provider "$provider_response" "Test${name}Provider" "$provider_type" "https://api.${provider_type}.com/v1" "$model"

    # Get the created LLM provider
    echo "Getting the created $name LLM provider"
    get_provider_response=$(make_request GET "/llm/providers/$provider_id" "" "$token" "200")

    # Validate the retrieved LLM provider
    validate_llm_provider "$get_provider_response" "Test${name}Provider" "$provider_type" "https://api.${provider_type}.com/v1" "$model"

    # Test LLM chat
    echo "Testing $name LLM chat"
    chat_response=$(make_request POST "/llm/chat" '{
        "provider_id": "'"$provider_id"'",
        "messages": [
            {"role": "user", "content": "Hello, how are you?"}
        ]
    }' "$token" "200")

    # Create a user LLM config
    echo "Creating a user LLM config for $name"
    config_response=$(make_request POST "/chat/user-llm-configs" '{
        "user_id": "'"$user_id"'",
        "provider_id": "'"$provider_id"'",
        "api_key_id": "'"$api_key_id"'"
    }' "$token" "201")
    config_id=$(echo "$config_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
    echo "$name Config ID: $config_id"

    if [ -z "$config_id" ]; then
        echo "Failed to create $name user LLM config. Skipping further tests."
        return
    fi

    # Get the created user LLM config
    echo "Getting the created $name user LLM config"
    make_request GET "/chat/user-llm-configs/$config_id" "" "$token" "200"

    # Delete the user LLM config
    echo "Deleting the $name user LLM config"
    make_request DELETE "/chat/user-llm-configs/$config_id" "" "$token" "204"

    # Verify the user LLM config was deleted
    echo "Verifying the $name user LLM config was deleted"
    make_request GET "/chat/user-llm-configs/$config_id" "" "$token" "404"

    # Delete the LLM provider
    echo "Deleting the $name LLM provider"
    make_request DELETE "/llm/providers/$provider_id" "" "$token" "204"

    # Verify the LLM provider was deleted
    echo "Verifying the $name LLM provider was deleted"
    make_request GET "/llm/providers/$provider_id" "" "$token" "404"

    # Add a small delay before deleting the API key
    sleep 2

    # Delete the API key
    echo "Deleting the $name API key"
    delete_response=$(make_request DELETE "/api_keys/$api_key_id" "" "$token" "204")
    delete_status=$(echo "$delete_response" | tail -n 1)
    if [ "$delete_status" != "204" ]; then
        echo "Failed to delete $name API key. Status: $delete_status"
        echo "Response: $delete_response"
        echo "API Key ID: $api_key_id"
        echo "Attempting to retrieve the API key to check its status..."
        make_request GET "/api_keys/$api_key_id" "" "$token" "404"
    else
        # Verify the API key was deleted
        echo "Verifying the $name API key was deleted"
        make_request GET "/api_keys/$api_key_id" "" "$token" "404"
    fi
}

# Test each provider
test_provider "OpenAI"
test_provider "Anthropic"
test_provider "Cohere"

# Get all LLM providers
echo "Getting all LLM providers"
make_request GET "/llm/providers" "" "$token" "200"

# Test creating a user LLM config with an invalid provider ID (error case)
echo "Testing creation of user LLM config with invalid provider ID"
make_request POST "/chat/user-llm-configs" '{
    "user_id": "'"$user_id"'",
    "provider_id": "00000000-0000-0000-0000-000000000000",
    "api_key_id": "00000000-0000-0000-0000-000000000000"
}' "$token" "400"

# Test getting a non-existent user LLM config (error case)
echo "Testing retrieval of non-existent user LLM config"
make_request GET "/chat/user-llm-configs/00000000-0000-0000-0000-000000000000" "" "$token" "404"

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
