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

# Create a new amber entry
echo "Creating a new amber entry"
amber_response=$(make_request POST "/amber_store" '{"name": "test_amber", "data": "test_data"}' "$token" "201")
amber_id=$(echo "$amber_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Amber ID: $amber_id"

# Create a new config entry
echo "Creating a new config entry"
config_response=$(make_request POST "/configurations" '{"name": "test_config", "data": {"key": "value"}}' "$token" "201")
config_id=$(echo "$config_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Config ID: $config_id"

# Create a new docker entry
echo "Creating a new docker entry"
docker_response=$(make_request POST "/docker_files" '{"name": "test_docker", "content": "FROM ubuntu:latest"}' "$token" "201")
docker_id=$(echo "$docker_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Docker ID: $docker_id"

# Create a new job entry
echo "Creating a new job entry"
job_response=$(make_request POST "/jobs" "{\"uri\": \"some-uri\", \"config\": {\"key\": \"value\"}, \"amber_id\": \"$amber_id\", \"state_file_content\": \"some-content\", \"data_path\": \"some-path\", \"worker_type\": \"$docker_id\", \"triggers\": null, \"timers\": null, \"status\": \"pending\"}" "$token" "201")
job_id=$(echo "$job_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Job ID: $job_id"

# List job entries
echo -e "\n\n\nListing job entries"
make_request GET "/jobs" "" "$token" "200"

# Get job entry details
echo -e "\n\n\nGet job entry details"
make_request GET "/jobs/$job_id" "" "$token" "200"

# Update job entry
echo -e "\n\n\nUpdate job entry"
make_request PUT "/jobs/$job_id" "{\"status\": \"running\"}" "$token" "200"

# Delete job entry
echo -e "\n\n\nDelete job entry"
make_request DELETE "/jobs/$job_id" "" "$token" "204"

# Delete docker entry
echo -e "\n\n\nDelete docker entry"
make_request DELETE "/docker_files/$docker_id" "" "$token" "204"

# Delete config entry
echo -e "\n\n\nDelete config entry"
make_request DELETE "/configurations/$config_id" "" "$token" "204"

# Delete amber entry
echo -e "\n\n\nDelete amber entry"
make_request DELETE "/amber_store/$amber_id" "" "$token" "204"

# Delete user
echo -e "\n\n\nDelete user"
make_request DELETE "/users/$user_id" "" "$token" "204"

# Summary
echo -e "\n\n\nTest Summary"
echo "Total Tests: $TOTAL_TESTS"
echo "Passed Tests: $PASSED_TESTS"
echo "Failed Tests: $((TOTAL_TESTS - PASSED_TESTS))"