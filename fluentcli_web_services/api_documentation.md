# API Documentation

## Chat Endpoints

All chat endpoints require authentication.

### Create Conversation

- **Method:** POST
- **Endpoint:** `/chat/conversations`
- **Request Body:**

  ```json
  {
    "user_id": "UUID",
    "title": "string"
  }
  ```

- **Response:** Returns the created conversation object
- **Possible Errors:**
  - 400 Bad Request: Invalid input
  - 500 Internal Server Error: Server-side error

### Get Conversation

- **Method:** GET
- **Endpoint:** `/chat/conversations/{id}`
- **Parameters:**
  - `id`: UUID of the conversation
- **Response:** Returns the conversation object
- **Possible Errors:**
  - 404 Not Found: Conversation not found
  - 500 Internal Server Error: Server-side error

### Create Message

- **Method:** POST
- **Endpoint:** `/chat/messages`
- **Request Body:**

  ```json
  {
    "conversation_id": "UUID",
    "role": "string",
    "content": "string"
  }
  ```

- **Response:** Returns the created message object
- **Possible Errors:**
  - 400 Bad Request: Invalid input
  - 404 Not Found: Conversation not found
  - 500 Internal Server Error: Server-side error

### Get Messages

- **Method:** GET
- **Endpoint:** `/chat/conversations/{id}/messages`
- **Parameters:**
  - `id`: UUID of the conversation
- **Response:** Returns an array of message objects
- **Possible Errors:**
  - 404 Not Found: Conversation not found
  - 500 Internal Server Error: Server-side error

### Create Attachment

- **Method:** POST
- **Endpoint:** `/chat/attachments`
- **Request Body:**

  ```json
  {
    "message_id": "UUID",
    "file_type": "string",
    "file_path": "string"
  }
  ```

- **Response:** Returns the created attachment object
- **Possible Errors:**
  - 400 Bad Request: Invalid input
  - 404 Not Found: Message not found
  - 500 Internal Server Error: Server-side error

### Get Attachments

- **Method:** GET
- **Endpoint:** `/chat/messages/{id}/attachments`
- **Parameters:**
  - `id`: UUID of the message
- **Response:** Returns an array of attachment objects
- **Possible Errors:**
  - 404 Not Found: Message not found
  - 500 Internal Server Error: Server-side error

### Create LLM Provider

- **Method:** POST
- **Endpoint:** `/chat/llm-providers`
- **Request Body:**

  ```json
  {
    "name": "string",
    "api_endpoint": "string"
  }
  ```

- **Response:** Returns the created LLM provider object
- **Possible Errors:**
  - 400 Bad Request: Invalid input
  - 500 Internal Server Error: Server-side error

### Get LLM Provider

- **Method:** GET
- **Endpoint:** `/chat/llm-providers/{id}`
- **Parameters:**
  - `id`: UUID of the LLM provider
- **Response:** Returns the LLM provider object
- **Possible Errors:**
  - 404 Not Found: LLM provider not found
  - 500 Internal Server Error: Server-side error

### Create User LLM Config

- **Method:** POST
- **Endpoint:** `/chat/user-llm-configs`
- **Request Body:**

  ```json
  {
    "user_id": "UUID",
    "provider_id": "UUID",
    "api_key": "string"
  }
  ```

- **Response:** Returns the created user LLM config object
- **Possible Errors:**
  - 400 Bad Request: Invalid input
  - 404 Not Found: User or LLM provider not found
  - 500 Internal Server Error: Server-side error

### Get User LLM Config

- **Method:** GET
- **Endpoint:** `/chat/user-llm-configs/{user_id}/{provider_id}`
- **Parameters:**
  - `user_id`: UUID of the user
  - `provider_id`: UUID of the LLM provider
- **Response:** Returns the user LLM config object
- **Possible Errors:**
  - 404 Not Found: User LLM config not found
  - 500 Internal Server Error: Server-side error

Note: All endpoints require authentication. Make sure to include the appropriate authentication headers with each request.
