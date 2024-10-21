# FluentCLI Web Services (pre-alpha)

FluentCLI Web Services is a comprehensive platform designed to provide a user-friendly interface for managing and executing FluentCLI commands. It consists of a Rust-based backend, a PostgreSQL database for data storage, and a Tauri-based frontend for both desktop and web applications.

## Table of Contents

1. [Features](#features)
2. [Architecture](#architecture)
3. [Getting Started](#getting-started)
4. [Usage](#usage)
5. [Chat System](#chat-system)
6. [LLM Providers](#llm-providers)
7. [Running Tests](#running-tests)
8. [Contributing](#contributing)
9. [License](#license)

## Features

* **User-friendly interface:** Manage FluentCLI jobs and configurations through an intuitive interface similar to Visual Studio Code.
* **Secure data management:** Securely store and manage user data, including API keys, Amber Store, and Vault Store.
* **Flexible job configuration:** Configure jobs with various parameters, including worker type, data path, triggers, and timers.
* **Pipeline management:** Create and manage pipelines with multiple stages and flags.
* **Job monitoring and management:** Monitor job status, view logs, and manage running jobs.
* **Cross-platform compatibility:** Access the platform through a desktop application (built with Tauri) or a web application.
* **Chat system:** Integrated chat functionality for user communication and AI assistance.
* **Multiple LLM providers:** Support for various Language Model providers, including GPT, Claude, Gemini, and more.

## Architecture

The platform consists of three main components:

* **Backend (Rust):**
  * Web server (Actix Web) handles API requests from the frontend.
  * Data access layer (Diesel ORM) interacts with the PostgreSQL database.
  * FluentCLI integration executes FluentCLI commands and manages job execution.
  * LLM service integrates multiple AI providers for chat functionality.
* **Database (PostgreSQL):**
  * Stores user data, job configurations, execution history, and chat data.
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
8. **Use the chat system:**
   * Use the API endpoints under `/chat` to create conversations, send messages, and manage attachments.

## Chat System

The chat system provides the following functionality:

* Create and manage conversations
* Send and receive messages within conversations
* Attach files to messages
* Configure and use different LLM (Language Model) providers
* Manage user-specific LLM configurations
* Stream responses from AI models for real-time interaction

To use the chat system, refer to the API documentation for detailed information on the available endpoints and their usage.

## LLM Providers

FluentCLI Web Services supports multiple Language Model providers, each with its own implementation and configuration. The following providers are currently supported:

1. **GPT (OpenAI):** Utilizes OpenAI's GPT models for natural language processing tasks.
2. **Claude (Anthropic):** Integrates Anthropic's Claude model for advanced language understanding and generation.
3. **Gemini (Google):** Implements Google's Gemini model, offering state-of-the-art language capabilities.
4. **Command (Cohere):** Incorporates Cohere's Command model for various language tasks.
5. **DALL-E:** Provides image generation capabilities using OpenAI's DALL-E model.
6. **Perplexity:** Integrates the Perplexity API for additional language processing features.

Each provider is implemented as a separate module in the `src/services/llm_providers` directory. The `GeminiProvider`, for example, is implemented in `src/services/llm_providers/gemini.rs` and includes the following key features:

* Streaming support for real-time response generation
* Customizable configuration options (temperature, top_k, top_p, max_tokens)
* Error handling and logging for robust operation

To add or modify LLM providers, refer to the existing implementations and follow the `LLMProviderTrait` interface.

## Running Tests

To run the tests for the application, use the following command:

```bash
./run_tests.sh
```

This command will execute all the test scripts, including:

* Chat service tests
* API key service tests
* Configuration service tests
* LLM provider tests
* And more...

Ensure all tests pass before submitting pull requests or deploying to production.

## Contributing

Contributions are welcome! Please follow these steps to contribute:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes and commit them with clear, descriptive messages
4. Push your changes to your fork
5. Submit a pull request to the main repository

Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for detailed guidelines on code style, testing requirements, and the pull request process.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

* The FluentCLI team for their excellent CLI tool
* The Rust community for their robust and efficient programming language
* The Tauri team for their cross-platform application framework
* All contributors who have helped shape and improve this project

For any questions, issues, or feature requests, please open an issue on the GitHub repository or contact the maintainers directly.
