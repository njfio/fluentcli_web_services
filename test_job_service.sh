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
name: my_pipeline
steps:
  - !Command
      name: create_story
      command: fluent openai-mini "'${input}'" -o temperature=1
      save_output: raw_data

  - !ShellCommand
      name: create_initial_image_prompt
      command: |
        fluent openai-mini 'create an image prompt that captures the essence of this text.  Only output the prompt.' -o temperature=1  <<EOT
        ${raw_data}
        EOT
      save_output: initial_image_prompt_data

  - !ShellCommand
    name: create_refined_image_prompt
    command: |
      fluent cohere 'refine this image prompt to produce a masterpiece that captures the essence of the text.  It must
      be less than 1000 characters..  Only output the prompt.' <<EOT
      ${initial_image_prompt_data}
      EOT
    save_output: image_prompt_data


  - !ShellCommand
      name: generate_images
      command: |
        fluent dalleVertical '' --download-media /shared_tmp/state_store <<EOT &
        ${image_prompt_data}
        EOT
        pid1=$!

        fluent leonardoVertical '' --download-media /shared_tmp/state_store <<EOT &
        ${image_prompt_data}
        EOT
        pid2=$!

        fluent stabilityUltraVertical '' --download-media /shared_tmp/state_store <<EOT &
        ${image_prompt_data}
        EOT
        pid3=$!

        wait $pid1 $pid2 $pid3
      save_output: image_data


  - !ShellCommand
      name: extract_summary
      command: |
        fluent sonnet3.5 'summarize the sentiment of this text in less than 3 words.  Only output the words.' <<EOT & 
        ${raw_data}
        EOT
      save_output: sentiment_data

  - !ShellCommand
      name: extract_semantics
      command: |
        fluent llama3-groq "summarize the semantic meaning of this text in less than 5 words.  Only output the words." <<EOT &
        ${raw_data}
        EOT
      save_output: semantic_data

  - !ShellCommand
      name: extract_triples
      command: |
        fluent openai-mini "give me an output of all the meaningful triples in this text.  Only output the cypher in Neo4j format. use single quotes" --parse-code <<EOT &
        ${raw_data}
        EOT
      save_output: triples_data

  - !ShellCommand
    name: add_triples
    command: |
      fluent neo4j --generate-cypher "create a cypher that adds these triples to the graph,  ${triples_data}"
    save_output: add_triples_data

  - !ShellCommand
      name: extract_theme
      command: |
        fluent  llama3-groq  'give me up to 5 words describing the theme of this text, output as a comma-separated list:' <<EOT &
        ${raw_data}
        EOT
      save_output: theme_data

  - !ShellCommand
      name: extract_3_keywords
      command: |
        fluent llama3-groq "Output the 3 keywords in a comma seperated list.  Output the list only."  <<EOT &
        ${raw_data}
        EOT
      save_output: 3_keywords_data

  - !ShellCommand
      name: sentiment_number
      command: |
        fluent gemma-groq 'on a decimal scale of -1.0 to 1.0 grade what the sentiment of this text, output the number only' <<EOT &
        ${raw_data}
        EOT
      save_output: sentiment_number_data

  - !ShellCommand
      name: trending_sentiment
      command: |
        fluent llama3-groq 'analyze the trending sentiment of this text, only output the trending sentiment, no formatting' <<EOT &
        ${raw_data}
        EOT
      save_output: trending_sentiment_data

  - !ShellCommand
      name: debug_variables
      command: |
        echo <<"""EOT"""
          raw_data: ${raw_data}
        
          theme_data: ${theme_data}
        EOT
      save_output: debug_output_data

  - !ShellCommand
      name: count_theme_words
      command: |
        wc -w <<< "${theme_data}" | awk '{print $1}'
      save_output: word_count

  - !Condition
      name: validate_data
      condition: "[ ${word_count} -le 5 ]"
      if_true: |
        echo "Theme data is correct"
      if_false: |
        echo "Theme data is not correct"

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