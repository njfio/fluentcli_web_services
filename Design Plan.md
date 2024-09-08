FluentCLI Web Services

Rust Based Backend and webserver
Unknown FrontEnd but should be dynamic and sexy something like a visual studio code interface.
Postgres data store


- User Account
    - ID
    - Secure API Keys
    - Secure Amber Store
        - Amber Store CRUD
    - Secure Vault Store
        - Secure Vault CRUD
    - Configuration Store
        - Configuration CRUD
    - Pipeline Store
        - Pipeline CRUD
    - Docker Files
        - Docker Files CRUD
    - ActiveWorkers
        - Activate/Deactivate Workers 
- Job Store
    - ID
    - URI
    - Config
    - AmberID
    - StateFileContent
    - DataPath
    - WorkerType (Docker File)
    - Triggers?
    - Timers?

- User Interface
    - Admin
        - API Keys and Secure Vault Stores
    - Studio
        - WorkerType (ActiveWorkers)
        - Config - Load, Create, Delete, Save
        - Engine
            - Expanding Fields For Values and Flags
        - Input Request
        - Pipeline - Load, Create, Delete, Save
            - Flags
        - Job List
            - Expandable
            - Parameterizable
            - URI
            - Status
            - Data Outputs



# FluentCLI Web Services: Revised Design Specification

## 1. Project Overview

FluentCLI Web Services is a platform designed to provide a user-friendly interface for managing and executing FluentCLI commands. It consists of a Rust-based backend, a PostgreSQL database for data storage, and a Tauri-based frontend for both desktop and web applications.

## 2. Project Goals

- Create a dynamic and intuitive user interface similar to Visual Studio Code.
- Provide secure storage and management of user data, including API keys and configurations.
- Enable flexible job configuration and pipeline management.
- Facilitate easy monitoring and management of FluentCLI jobs.
- Support both desktop and web-based access through Tauri.

## 3. Architecture Overview

### 3.1 Backend (Rust)
- Web Server: Handles API requests from the frontend.
- Data Access Layer: Interacts with the PostgreSQL database.
- FluentCLI Integration: Executes FluentCLI commands and manages job execution.

### 3.2 Database (PostgreSQL)
Stores user data, job configurations, and execution history.

### 3.3 Frontend (Tauri)
- User Interface: HTML, CSS, JavaScript
- Tauri API: Enables communication with the backend and access to native desktop features.

## 4. Data Model

### 4.1 User Account
- ID: UUID
- Secure API Keys: String Array
- Secure Amber Store: JSON (Encrypted)
- Secure Vault Store: JSON (Encrypted)
- Configuration Store: JSON
- Pipeline Store: JSON Array
- Docker Files: String Array
- ActiveWorkers: Boolean Array

### 4.2 Job Store
- ID: UUID
- URI: String
- Config: JSON
- AmberID: UUID
- StateFileContent: String
- DataPath: String
- WorkerType: String (Docker File reference)
- Triggers: JSON (Optional)
- Timers: JSON (Optional)

## 5. API Endpoints

### 5.1 User Management
- POST /users: Create a new user account
- GET /users/{id}: Retrieve user details
- PUT /users/{id}: Update user account
- DELETE /users/{id}: Delete user account

- POST /users/{id}/api_keys: Add new API key
- GET /users/{id}/api_keys: List all API keys
- DELETE /users/{id}/api_keys/{key_id}: Remove an API key

- CRUD operations for Amber Store, Vault Store, Configurations, Pipelines, Docker Files, and Workers

### 5.2 Job Management
- POST /jobs: Create a new job
- GET /jobs: List all jobs (with filtering options)
- GET /jobs/{id}: Retrieve job details
- PUT /jobs/{id}: Update job configuration
- DELETE /jobs/{id}: Delete a job

- POST /jobs/{id}/start: Start a job
- POST /jobs/{id}/stop: Stop a running job
- GET /jobs/{id}/status: Get job status
- GET /jobs/{id}/output: Retrieve job output data

## 6. Frontend User Interface

### 6.1 Admin Panel
- API Keys Management
- Secure Vault Management

### 6.2 Studio
- WorkerType Selection (ActiveWorkers)
- Configuration Editor
    - Load, Create, Delete, Save configurations
    - Expanding fields for values and flags
- Engine Settings
- Input Request Builder
- Pipeline Editor
    - Load, Create, Delete, Save pipelines
    - Define pipeline stages and flags
- Job List
    - Expandable job details
    - Parameterizable job configurations
    - Display URI, status, and data outputs
    - Sorting and filtering options

## 7. Technology Stack

- Backend: Rust (Actix Web, Diesel ORM)
- Database: PostgreSQL
- Frontend: Tauri (HTML, CSS, JavaScript)
- Containerization: Docker

## 8. Security Considerations

- Implement robust API authentication using API keys
- Encrypt sensitive data in Amber Store and Secure Vault
- Use secure protocols (HTTPS) for all communications
- Implement proper access controls and input validation

## 9. Rust Implementation Details

### 9.1 Backend Structure

```rust
fluentcli_web_services/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── models/
│   │   ├── user.rs
│   │   ├── job.rs
│   │   └── pipeline.rs
│   ├── handlers/
│   │   ├── user.rs
│   │   ├── job.rs
│   │   └── pipeline.rs
│   ├── services/
│   │   ├── user_service.rs
│   │   ├── job_service.rs
│   │   └── fluentcli_service.rs
│   ├── db/
│   │   ├── mod.rs
│   │   └── schema.rs
│   └── utils/
│       ├── error.rs
│       └── auth.rs
```

### 9.2 Key Rust Features and Libraries

- Actix Web: For building the RESTful API
- Diesel: ORM for database interactions
- Serde: For serialization and deserialization
- Tokio: Asynchronous runtime for concurrent operations
- jsonwebtoken: For handling JWT authentication
- bcrypt: For password hashing

### 9.3 Error Handling

Implement a custom error type for the application:

```rust
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(diesel::result::Error),
    ValidationError(String),
    AuthenticationError,
    FluentCLIError(String),
    NotFound,
    InternalServerError,
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::AuthenticationError => write!(f, "Authentication failed"),
            AppError::FluentCLIError(msg) => write!(f, "FluentCLI error: {}", msg),
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}
```

### 9.4 FluentCLI Integration

Create a service to interact with FluentCLI:

```rust
use std::process::Command;

pub struct FluentCLIService;

impl FluentCLIService {
    pub fn execute_command(&self, command: &str) -> Result<String, AppError> {
        let output = Command::new("fluentcli")
            .args(command.split_whitespace())
            .output()
            .map_err(|e| AppError::FluentCLIError(e.to_string()))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(AppError::FluentCLIError(String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }
}
```

## 10. Frontend Implementation (Tauri)

### 10.1 Project Structure

```
fluentcli_web_services/
├── src/
│   ├── db/
│   ├── error.rs
│   ├── handlers/
│   ├── main.rs
│   ├── models/
│   ├── routes.rs
│   ├── schema.rs
│   ├── services/
│   └── utils/
├── src-tauri/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   ├── views/
│   │   ├── store/
│   │   ├── router/
│   │   ├── App.vue
│   │   └── main.ts
│   ├── public/
│   │   └── index.html
│   └── package.json
├── Cargo.toml
└── Dockerfile
```

### 10.2 Key Frontend Technologies

- Vue.js: For building the user interface
- Vuex: For state management
- Vue Router: For navigation
- Axios: For making API requests to the backend
- CodeMirror: For code editing functionality in configuration and pipeline editors

## 11. Development Roadmap

1. Set up the Rust backend project structure and implement basic API endpoints.
2. Create the PostgreSQL database schema and implement data access layer.
3. Implement user authentication and authorization.
4. Develop the FluentCLI integration service.
5. Set up the Tauri project and implement the basic UI structure.
6. Implement the Admin Panel functionality.
7. Develop the Studio components (WorkerType selection, Configuration Editor, etc.).
8. Implement job management and monitoring features.
9. Integrate frontend with backend API.
10. Implement error handling and logging throughout the application.
11. Conduct thorough testing (unit tests, integration tests, and end-to-end tests).
12. Optimize performance and refine the user interface.
13. Prepare documentation and deployment instructions.

## 12. Conclusion

This revised specification provides a comprehensive plan for developing the FluentCLI Web Services platform. It focuses on creating a user-friendly interface around FluentCLI functionality, with a robust backend for data management and job execution. The use of Rust for the backend and Tauri for the frontend ensures high performance and cross-platform compatibility.