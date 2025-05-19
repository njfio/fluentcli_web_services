#!/bin/bash

BASE_URL="http://localhost:8000"

# Function to make API requests
make_request() {
    local method=$1
    local endpoint=$2
    local data=$3
    
    echo "Testing $method $endpoint"
    if [ -n "$data" ]; then
        curl -X $method -H "Content-Type: application/json" -d "$data" $BASE_URL$endpoint
    else
        curl -X $method $BASE_URL$endpoint
    fi
    echo -e "\n"
}

# User routes
make_request POST "/users/login" '{"username": "testuser", "password": "testpass"}'
make_request POST "/users" '{"username": "newuser", "email": "newuser@example.com", "password": "newpass"}'
make_request GET "/users"

# API Key routes
make_request POST "/users/1/api_keys" '{"name": "Test API Key"}'
make_request GET "/users/1/api_keys"
make_request DELETE "/users/1/api_keys/1"

# Amber Store routes
make_request POST "/users/1/amber_store" '{"key": "test_key", "value": "test_value"}'
make_request GET "/users/1/amber_store"
make_request GET "/users/1/amber_store/test_key"
make_request PUT "/users/1/amber_store/test_key" '{"value": "updated_value"}'
make_request DELETE "/users/1/amber_store/test_key"

# Vault Store routes
make_request POST "/users/1/vault_store" '{"key": "test_key", "value": "test_value"}'
make_request GET "/users/1/vault_store"
make_request GET "/users/1/vault_store/test_key"
make_request PUT "/users/1/vault_store/test_key" '{"value": "updated_value"}'
make_request DELETE "/users/1/vault_store/test_key"

# Configuration routes
make_request POST "/configurations" '{"name": "Test Config", "data": {}}'
make_request GET "/configurations"
make_request GET "/configurations/1"
make_request PUT "/configurations/1" '{"name": "Updated Config", "data": {}}'
make_request DELETE "/configurations/1"

# Pipeline routes
make_request POST "/pipelines" '{"name": "Test Pipeline", "data": {}}'
make_request GET "/pipelines"
make_request GET "/pipelines/1"
make_request PUT "/pipelines/1" '{"name": "Updated Pipeline", "data": {}}'
make_request DELETE "/pipelines/1"

# Docker File routes
make_request POST "/docker_files" '{"name": "Test Dockerfile", "content": "FROM alpine:latest"}'
make_request GET "/docker_files"
make_request GET "/docker_files/1"
make_request PUT "/docker_files/1" '{"name": "Updated Dockerfile", "content": "FROM ubuntu:latest"}'
make_request DELETE "/docker_files/1"

# Worker routes
make_request GET "/workers"
make_request POST "/workers/1/activate"
make_request POST "/workers/1/deactivate"

# Job routes
make_request POST "/jobs" '{"uri": "test://uri", "config": {}, "worker_type": "test_worker"}'
make_request GET "/jobs"
make_request GET "/jobs/1"
make_request PUT "/jobs/1" '{"uri": "updated://uri", "config": {}, "worker_type": "updated_worker"}'
make_request DELETE "/jobs/1"
make_request POST "/jobs/1/start"
make_request POST "/jobs/1/stop"
make_request GET "/jobs/1/status"
make_request GET "/jobs/1/output"
make_request GET "/jobs/1/logs"

echo "API endpoint testing completed."