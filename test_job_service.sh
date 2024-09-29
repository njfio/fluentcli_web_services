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

# Create a new amber store entry
echo "Creating a new amber store entry"
amber_response=$(make_request POST "/amber_store" '{"name": "Test Entry", "data": {"key": "value"}, "secure_key_hash": "secretkey123"}' "$token" "201")
amber_id=$(echo "$amber_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Amber Store ID: $amber_id"


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

# Create a new pipeline entry
echo "Creating a new pipeline entry"
pipeline_content=$(cat <<EOF
name: parallel_and_timeout_example
steps:
  - !Parallel
    name: concurrent_operations
    steps:
      - !ShellCommand
        name: task1
        command: sleep 2 && echo "Task 1 completed"
        save_output: task1_result
      - !ShellCommand
        name: task2
        command: sleep 1 && echo "Task 2 completed"
        save_output: task2_result
      - !Timeout
        name: timed_task
        duration: 3
        step:
          !ShellCommand
          name: long_task
          command: sleep 5 && echo "This should time out"
          save_output: long_task_result

  - !PrintOutput
    name: final_output
    value: |
      Parallel Execution Results:
        Task 1: ${task1_result}
        Task 2: ${task2_result}
        Timed Task: ${long_task_result}
      Errors (if any):
        ${error_0}
        ${error_1}
        ${error_2}
EOF
)

pipeline_response=$(make_request POST "/pipelines" "{\"name\": \"test_pipeline\", \"data\": $(echo "$pipeline_content" | jq -R -s '.')}" "$token" "201")
pipeline_id=$(echo "$pipeline_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Pipeline ID: $pipeline_id"

# Create a new job entry
echo "Creating a new job entry"
job_response=$(make_request POST "/jobs" "{\"config\": \"$config_id\", \"amber_id\": \"$amber_id\", \"state_file_content\": \"{\\\"content\\\": \\\"some-content\\\"}\", \"data_path\": \"some-path\", \"worker_type\": \"$docker_id\", \"triggers\": null, \"timers\": null, \"status\": \"pending\", \"pipeline_id\": \"$pipeline_id\"}" "$token" "201")
job_id=$(echo "$job_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4 | head -n 1)
echo "Job ID: $job_id"
echo "Job Response: $job_response"

# Start the job
echo -e "\n\n\nStarting the job"
make_request POST "/jobs/$job_id/start" "" "$token" "200"

echo -e "\n\n\nChecking job status"
# Checking job status
max_attempts=30
attempt=0
while [ $attempt -lt $max_attempts ]; do
    status_response=$(make_request GET "/jobs/$job_id/status" "" "$token" "200")
    echo "Status Response: $status_response"
    
    # Extract the response body correctly
    body=$(echo "$status_response" | grep -o '==== Response Body ====' -A 1 | tail -n 1)
    status=$(echo "$body" | tr -d '[:space:]' | tr -d '"')
    
    echo "Current status: $status"
    
    case $status in
        "completed" | "failed")
            echo "Condition met: status is completed or failed"
            echo "Job finished with status: $status"
            break
            ;;
        "running")
            echo "Condition met: status is running"
            echo "Job status: $status. Waiting..."
            sleep 30
            ;;
        *)
            echo "Condition met: status is neither completed, failed, nor running"
            echo "Job status: $status. Waiting..."
            sleep 5
            ;;
    esac
    
    attempt=$((attempt + 1))
    echo "Attempt: $attempt"
done

if [ "$status" != "completed" ] && [ "$status" != "failed" ]; then
    echo "Final condition: status is not completed and not failed"
    echo "Job did not complete within the expected time."
    exit 1
fi

# Get job output
echo -e "\n\n\nGetting job output"
make_request GET "/jobs/$job_id/output" "" "$token" "200"

# Get job logs
echo -e "\n\n\nGetting job logs"
make_request GET "/jobs/$job_id/logs" "" "$token" "200"

# Update job entry
echo -e "\n\n\nUpdate job entry"
make_request PUT "/jobs/$job_id" "{\"status\": \"archived\"}" "$token" "200"

# Stop the job (this should fail as the job is already completed)
echo -e "\n\n\nAttempting to stop the completed job"
make_request POST "/jobs/$job_id/stop" "" "$token" "400"

# Delete job entry
#echo -e "\n\n\nDelete job entry"
#make_request DELETE "/jobs/$job_id" "" "$token" "204"

# Delete docker entry
#echo -e "\n\n\nDelete docker entry"
#make_request DELETE "/docker_files/$docker_id" "" "$token" "204"

# Delete config entry
#echo -e "\n\n\nDelete config entry"
#make_request DELETE "/configurations/$config_id" "" "$token" "204"

# Delete amber entry
#echo -e "\n\n\nDelete amber entry"
#make_request DELETE "/amber_store/$amber_id" "" "$token" "204"

# Delete pipeline entry
#echo -e "\n\n\nDelete pipeline entry"
#make_request DELETE "/pipelines/$pipeline_id" "" "$token" "204"

# Delete user
echo -e "\n\n\nDelete user"
make_request DELETE "/users/$user_id" "" "$token" "204"

# Summary
echo -e "\n\n\nTest Summary"
echo "Total Tests: $TOTAL_TESTS"
echo "Passed Tests: $PASSED_TESTS"
echo "Failed Tests: $((TOTAL_TESTS - PASSED_TESTS))"