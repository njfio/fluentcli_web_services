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

    response=$(curl -s -w "\n%{http_code}" -X $method "${headers[@]}" -d "$data" "$BASE_URL$endpoint")
    
    status_code=$(echo "$response" | tail -n 1)
    body=$(echo "$response" | sed '$d')
    
    if [ "$status_code" = "$expected_status" ]; then
        echo "Test passed: $method $endpoint (Status: $status_code)"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "Test failed: $method $endpoint (Expected: $expected_status, Got: $status_code)"
    fi
    
    echo "$body"
}

# Create a new user
echo "Creating a new user"
response=$(make_request POST "/users" '{"username": "testuser", "email": "testuser@example.com", "password": "testpass"}' "" "201")
user_id=$(echo "$response" | grep -o '"id":"[^"]*' | cut -d'"' -f4)

# Login with the new user
echo "Logging in with the new user"
login_response=$(make_request POST "/users/login" '{"username": "testuser", "password": "testpass"}' "" "200")
token=$(echo "$login_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4)

# Test FluentCLI service
echo "Testing FluentCLI service"
fluentcli_response=$(make_request POST "/fluentcli/execute" '{"command": "openai", "args": ["python example script"]}' "$token" "200")
echo "FluentCLI response: $fluentcli_response"

# Check the FluentCLI response
if echo "$fluentcli_response" | grep -q '"output":' && echo "$fluentcli_response" | grep -q '"error":null' && echo "$fluentcli_response" | grep -q '"exit_code":0'; then
    echo "FluentCLI test passed: Correct response structure received"
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    echo "FluentCLI test failed: Incorrect response structure"
fi

echo ""
echo "Testing FluentCLI service with pipeline"
fluentcli_response=$(make_request POST "/fluentcli/execute" '{"command": "openai", "args": ["pipeline", "--file", "/.fluent/example_pipelines/example_parallel_and_timeout.yaml", "--input", "testing", "--run-id", "testing", "--json-output", "--force-fresh"]}' "$token" "200")
echo "FluentCLI response: $fluentcli_response"

# Check the FluentCLI response
if echo "$fluentcli_response" | grep -q '"output":' && echo "$fluentcli_response" | grep -q '"error":null' && echo "$fluentcli_response" | grep -q '"exit_code":0'; then
    echo "FluentCLI pipeline test passed: Correct response structure received"
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    echo "FluentCLI pipeline test failed: Incorrect response structure"
fi


echo ""
echo "Testing FluentCLI service with pipeline"
fluentcli_response=$(make_request POST "/fluentcli/execute" '{"command": "openai", "args": ["pipeline", "--file", "/.fluent/example_pipelines/example_detailed_article_generation.yaml", "--input", "Who will be elected president of the united states in 2024", "--run-id", "testing123", "--json-output"]}' "$token" "200")
echo "FluentCLI response: $fluentcli_response"

# Check the FluentCLI response
if echo "$fluentcli_response" | grep -q '"output":' && echo "$fluentcli_response" | grep -q '"error":null' && echo "$fluentcli_response" | grep -q '"exit_code":0'; then
    echo "FluentCLI pipeline test passed: Correct response structure received"
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    echo "FluentCLI pipeline test failed: Incorrect response structure"
fi

TOTAL_TESTS=$((TOTAL_TESTS + 1))

TOTAL_TESTS=$((TOTAL_TESTS + 1))

# Delete the user
echo "Deleting the user"
make_request DELETE "/users/$user_id" "" "$token" "204"

# Print summary
echo "Test Summary:"
echo "Total tests: $TOTAL_TESTS"
echo "Passed tests: $PASSED_TESTS"
echo "Failed tests: $((TOTAL_TESTS - PASSED_TESTS))"
echo "Success rate: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%"
echo "FluentCLI service testing completed."