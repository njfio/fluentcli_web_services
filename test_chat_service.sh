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
provider_response=$(make_request POST "/chat/llm-providers" '{"name": "Test Provider", "api_endpoint": "https://api.testprovider.com"}' "$token" "201")
provider_id=$(echo "$provider_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Provider ID: $provider_id"

if [ -z "$provider_id" ]; then
    echo "Failed to create LLM provider. Exiting."
    exit 1
fi

# Get the created LLM provider
echo "Getting the created LLM provider"
make_request GET "/chat/llm-providers/$provider_id" "" "$token" "200"

# Create a user LLM config
echo "Creating a user LLM config"
llm_config_response=$(make_request POST "/chat/user-llm-configs" '{"provider_id": "'"$provider_id"'", "api_key": "test_api_key"}' "$token" "201")
llm_config_id=$(echo "$llm_config_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "LLM Config ID: $llm_config_id"

if [ -z "$llm_config_id" ]; then
    echo "Failed to create user LLM config. Exiting."
    exit 1
fi

# Get the user LLM config
echo "Getting the user LLM config"
echo "Debug: user_id=$user_id, provider_id=$provider_id, llm_config_id=$llm_config_id"
make_request GET "/chat/user-llm-configs/$user_id/$provider_id" "" "$token" "200"

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