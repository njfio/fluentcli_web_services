#!/bin/bash

BASE_URL="http://localhost:8000"
TOTAL_TESTS=0
PASSED_TESTS=0

# ... (keep the existing make_request and print_result functions)
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
    body=$(echo "$response" | sed '$d')
    
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
TIMESTAMP=$(date +%s)
USERNAME="chatuser_${TIMESTAMP}"

# Create a new user
echo "Creating a new user"
user_response=$(make_request POST "/users" '{"username": "'"$USERNAME"'", "email": "'"$USERNAME@example.com"'", "password": "chatpass"}' "" "201")
user_id=$(echo "$user_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "User ID: $user_id"

if [ -z "$user_id" ]; then
    echo "Failed to create user. Exiting."
    exit 1
fi

# Login with the new user
echo "Logging in with the new user"
login_response=$(make_request POST "/users/login" '{"username": "'"$USERNAME"'", "password": "chatpass"}' "" "200")
token=$(echo "$login_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Token: $token"

if [ -z "$token" ]; then
    echo "Failed to login. Exiting."
    exit 1
fi

create_and_test_llm_provider() {
    local name=$1
    local provider_type=$2
    local api_endpoint=$3
    local model=$4

    # Get the API key from the environment variable
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

    echo "***API key for $name: $api_key"
    if [ -z "$api_key" ]; then
        echo "API key for $name is not set. Skipping tests."
        return
    fi

    # Create a new API key entry in the system
    echo "Creating a new API key entry for $name"
    api_key_response=$(make_request POST "/api_keys" '{
        "user_id": "'"$user_id"'",
        "name": "'"$name"'APIKey",
        "key_value": "'"$api_key"'"
    }' "$token" "201")
    api_key_id=$(echo "$api_key_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
    echo "$name API Key ID (system identifier): $api_key_id"

    if [ -z "$api_key_id" ]; then
        echo "Failed to create API key entry for $name. Skipping tests."
        return
    fi

    echo "Creating $name LLM provider"
    provider_response=$(make_request POST "/llm/providers" '{
        "user_id": "'"$user_id"'",
        "name": "'"$name"'",
        "provider_type": "'"$provider_type"'",
        "api_endpoint": "'"$api_endpoint"'",
        "supported_modalities": ["text"],
        "configuration": {
            "model": "'"$model"'",
            "max_tokens": 150
        }
    }' "$token" "201")
    provider_id=$(echo "$provider_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
    echo "$name Provider ID: $provider_id"

    if [ -z "$provider_id" ]; then
        echo "Failed to create $name LLM provider. Skipping tests."
        return
    fi

    # Get the created LLM provider
    echo "Getting the created $name LLM provider"
    make_request GET "/llm/providers/$provider_id" "" "$token" "200"

    # Create a user LLM config
    echo "Creating a user LLM config for $name"
    llm_config_response=$(make_request POST "/chat/user-llm-configs" '{
        "user_id": "'"$user_id"'",
        "provider_id": "'"$provider_id"'",
        "api_key_id": "'"$api_key_id"'"
    }' "$token" "201")
    llm_config_id=$(echo "$llm_config_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
    echo "$name LLM Config ID: $llm_config_id"

    if [ -z "$llm_config_id" ]; then
        echo "Failed to create user LLM config for $name. Skipping tests."
        return
    fi

    # Get the user LLM config
    echo "Getting the user LLM config for $name"
    make_request GET "/chat/user-llm-configs/$llm_config_id" "" "$token" "200"

    # Create a new conversation for this LLM provider test
    echo "Creating a new conversation for $name LLM test"
    conversation_response=$(make_request POST "/chat/conversations" '{"title": "Test Conversation for '"$name"'"}' "$token" "201")
    conversation_id=$(echo "$conversation_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
    echo "$name Test Conversation ID: $conversation_id"

    if [ -z "$conversation_id" ]; then
        echo "Failed to create conversation for $name. Skipping test."
        return
    fi

    # Test LLM chat
    echo "Testing $name LLM chat"
    chat_response=$(make_request POST "/llm/chat" '{
        "provider_id": "'"$provider_id"'",
        "conversation_id": "'"$conversation_id"'",
        "messages": [
            {"role": "user", "content": "What is the capital of France?"}
        ]
    }' "$token" "200")

    # Parse the chat response
    response_status=$(echo "$chat_response" | tail -n 1)
    echo "Response status: $response_status"
    response_body=$(echo "$chat_response" )

    if [ "$response_status" = "200" ]; then
        echo -e "\n==== $name Assistant Response ===="
        echo "Assistant response: $response_body"

        if [[ $response_body == *"Paris"* ]]; then
            echo "$name LLM Service test passed: Response contains 'Paris'"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            echo "$name LLM Service test failed: Response does not contain 'Paris'"
        fi
    else
        echo "$name LLM Service test failed: Unexpected status code $response_status"
        echo "Response body: $response_body"
    fi

    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    # Add a small delay to ensure the message is saved
    sleep 2

    # Check if the LLM response was saved to the database
    echo "Checking if $name LLM response was saved to the database"
    messages_response=$(make_request GET "/chat/conversations/$conversation_id/messages" "" "$token" "200")
    echo "Messages in the conversation:"
    echo "$messages_response"

    # Parse the messages response and check for the assistant message
    if echo "$messages_response" | grep -q '"role":"assistant"' && echo "$messages_response" | grep -q '"content":".*Paris.*"'; then
        echo "$name Database write test passed: LLM response found in the conversation messages"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "$name Database write test failed: LLM response not found in the conversation messages"
    fi

    TOTAL_TESTS=$((TOTAL_TESTS + 1))
}

# Test OpenAI provider
create_and_test_llm_provider "OpenAI" "gpt" "https://api.openai.com/v1/chat/completions" "gpt-3.5-turbo"

# Test Anthropic provider
create_and_test_llm_provider "Anthropic" "claude" "https://api.anthropic.com/v1/complete" "claude-2"

# Test Cohere provider
create_and_test_llm_provider "Cohere" "command" "https://api.cohere.ai/v1/chat" "command"

# Print final test results
echo -e "\n==== Test Results ===="
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
