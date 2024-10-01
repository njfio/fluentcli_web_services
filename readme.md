# FluentCLI Web Services (pre-alpha)

FluentCLI Web Services is a platform designed to provide a user-friendly interface for managing and executing FluentCLI commands. It consists of a Rust-based backend, a PostgreSQL database for data storage, and a Tauri-based frontend for both desktop and web applications.

## Features

* **User-friendly interface:** Manage FluentCLI jobs and configurations through an intuitive interface similar to Visual Studio Code.
* **Secure data management:** Securely store and manage user data, including API keys, Amber Store, and Vault Store.
* **Flexible job configuration:** Configure jobs with various parameters, including worker type, data path, triggers, and timers.
* **Pipeline management:** Create and manage pipelines with multiple stages and flags.
* **Job monitoring and management:** Monitor job status, view logs, and manage running jobs.
* **Cross-platform compatibility:** Access the platform through a desktop application (built with Tauri) or a web application.

## Architecture

The platform consists of three main components:

* **Backend (Rust):**
  * Web server (Actix Web) handles API requests from the frontend.
  * Data access layer (Diesel ORM) interacts with the PostgreSQL database.
  * FluentCLI integration executes FluentCLI commands and manages job execution.
* **Database (PostgreSQL):**
  * Stores user data, job configurations, and execution history.
* **Frontend (Tauri):**
  * User interface built with HTML, CSS, JavaScript, and Vue.js.
  * Tauri API enables communication with the backend and access to native desktop features.

## Getting Started

### Prerequisites

* Docker
* Docker Compose

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/fluentcli-web-services.git
   ```

2. Build and start the application using Docker Compose:

   ```bash
   docker-compose up -d
   ```

3. Access the application:
   * Desktop application: The Tauri application will be launched automatically.
   * Web application: Open your browser and navigate to `http://localhost:1420`.

## Usage

1. **Create a user account:**
   * Use the API endpoint `/users` to create a new user account.
2. **Manage API keys:**
   * Use the API endpoints under `/users/{id}/api_keys` to manage your API keys.
3. **Configure Amber Store and Vault Store:**
   * Use the API endpoints for Amber Store and Vault Store to manage your secure data.
4. **Create and manage configurations:**
   * Use the API endpoints under `/configurations` to create and manage configurations.
5. **Create and manage pipelines:**
   * Use the API endpoints under `/pipelines` to create and manage pipelines.
6. **Manage Docker files:**
   * Use the API endpoints under `/docker_files` to manage your Docker files.
7. **Create and manage jobs:**
   * Use the API endpoints under `/jobs` to create, manage, start, stop, and monitor jobs.

## Contributing

Contributions are welcome! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
