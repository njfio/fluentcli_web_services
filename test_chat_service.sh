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

# Generate a unique email
TIMESTAMP=$(date +%s)
EMAIL="chatuser_${TIMESTAMP}@example.com"

# Create a new user
echo "Creating a new user"
user_response=$(make_request POST "/users" '{"username": "chatuser_'"$TIMESTAMP"'", "email": "'"$EMAIL"'", "password": "chatpass"}' "" "201")
user_id=$(echo "$user_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "User ID: $user_id"

if [ -z "$user_id" ]; then
    echo "Failed to create user. Exiting."
    exit 1
fi

# Login with the new user
echo "Logging in with the new user"
login_response=$(make_request POST "/users/login" '{"username": "chatuser_'"$TIMESTAMP"'", "password": "chatpass"}' "" "200")
token=$(echo "$login_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Token: $token"

if [ -z "$token" ]; then
    echo "Failed to login. Exiting."
    exit 1
fi

# Create a new conversation
echo "Creating a new conversation"
conversation_response=$(make_request POST "/chat/conversations" '{"title": "Test Conversation"}' "$token" "201")
conversation_id=$(echo "$conversation_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Conversation ID: $conversation_id"

if [ -z "$conversation_id" ]; then
    echo "Failed to create conversation. Exiting."
    exit 1
fi

# Get the created conversation
echo "Getting the created conversation"
make_request GET "/chat/conversations/$conversation_id" "" "$token" "200"

# List all conversations for the user
echo "Listing all conversations for the user"
make_request GET "/chat/conversations" "" "$token" "200"

# Create a new message in the conversation
echo "Creating a new message"
message_response=$(make_request POST "/chat/messages" '{"conversation_id": "'"$conversation_id"'", "role": "user", "content": "Hello, this is a test message"}' "$token" "201")
message_id=$(echo "$message_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Message ID: $message_id"

if [ -z "$message_id" ]; then
    echo "Failed to create message. Exiting."
    exit 1
fi

# Get messages for the conversation
echo "Getting messages for the conversation"
make_request GET "/chat/conversations/$conversation_id/messages" "" "$token" "200"

# Create an attachment for the message
echo "Creating an attachment"
make_request POST "/chat/attachments" '{"message_id": "'"$message_id"'", "file_type": "text", "file_path": "/path/to/test/file.txt"}' "$token" "201"

# Get attachments for the message
echo "Getting attachments for the message"
make_request GET "/chat/messages/$message_id/attachments" "" "$token" "200"

# Create an LLM provider
echo "Creating an LLM provider"
provider_response=$(make_request POST "/llm/providers" '{
    "name": "OpenAI",
    "provider_type": "gpt",
    "api_endpoint": "https://api.openai.com/v1/chat/completions",
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

# Get the created LLM provider
echo "Getting the created LLM provider"
make_request GET "/llm/providers/$provider_id" "" "$token" "200"

# Create a user LLM config
echo "Creating a user LLM config"
llm_config_response=$(make_request POST "/chat/user-llm-configs" '{
    "user_id": "'"$user_id"'",
    "provider_id": "'"$provider_id"'",
    "api_key": "test_api_key"
}' "$token" "201")
llm_config_id=$(echo "$llm_config_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "LLM Config ID: $llm_config_id"

if [ -z "$llm_config_id" ]; then
    echo "Failed to create user LLM config. Exiting."
    exit 1
fi

# Get the user LLM config
echo "Getting the user LLM config"
make_request GET "/chat/user-llm-configs?user_id=$user_id&provider_id=$provider_id" "" "$token" "200"

# Test LLM chat
echo "Testing LLM chat"
chat_response=$(make_request POST "/llm/chat" '{
    "provider_id": "'"$provider_id"'",
    "conversation_id": "'"$conversation_id"'",
    "messages": [
        {"role": "user", "content": "What is the capital of France?"}
    ]
}' "$token" "200")

# Parse the chat response
assistant_message=$(echo "$chat_response" )
echo -e "\n==== Assistant Response ===="
echo "Assistant response: $assistant_message"

if [[ $assistant_message == *"Paris"* ]]; then
    echo "LLM Service test passed: Response contains 'Paris'"
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    echo "LLM Service test failed: Response does not contain 'Paris'"
    echo "Full response: $chat_response"
fi

TOTAL_TESTS=$((TOTAL_TESTS + 1))

# Check if the LLM response was saved to the database
echo "Checking if LLM response was saved to the database"
messages_response=$(make_request GET "/chat/conversations/$conversation_id/messages" "" "$token" "200")
echo "Messages in the conversation:"
echo "$messages_response"

# Parse the messages response and check for the assistant message
if echo "$messages_response" | grep -q '"role":"assistant"' && echo "$messages_response" | grep -q '"content":".*Paris.*"'; then
    echo "Database write test passed: LLM response found in the conversation messages"
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    echo "Database write test failed: LLM response not found in the conversation messages"
    echo "Full messages response: $messages_response"
fi

TOTAL_TESTS=$((TOTAL_TESTS + 1))

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