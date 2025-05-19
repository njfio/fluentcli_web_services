# FluentCLI Web Services (pre-alpha)

FluentCLI Web Services is a comprehensive platform designed to provide a user-friendly interface for managing and executing FluentCLI commands. It consists of a Rust-based backend, a PostgreSQL database for data storage, and a Tauri-based frontend for both desktop and web applications.

## Table of Contents

1. [Features](#features)
2. [Architecture](#architecture)
3. [Getting Started](#getting-started)
4. [Usage](#usage)
5. [Chat System](#chat-system)
6. [LLM Providers](#llm-providers)
7. [Image Generation](#image-generation)
8. [Running Tests](#running-tests)
9. [Contributing](#contributing)
10. [License](#license)

## Features

* **User-friendly interface:** Manage FluentCLI jobs and configurations through an intuitive interface similar to Visual Studio Code.
* **Secure data management:** Securely store and manage user data, including API keys, Amber Store, and Vault Store.
* **Flexible job configuration:** Configure jobs with various parameters, including worker type, data path, triggers, and timers.
* **Pipeline management:** Create and manage pipelines with multiple stages and flags.
* **Job monitoring and management:** Monitor job status, view logs, and manage running jobs.
* **Cross-platform compatibility:** Access the platform through a desktop application (built with Tauri) or a web application.
* **Chat system:** Integrated chat functionality for user communication and AI assistance.
* **Multiple LLM providers:** Support for various Language Model providers, including GPT, Claude, Gemini, and more.
* **Image generation:** Built-in support for AI image generation through multiple providers.
* **Attachment handling:** Robust system for managing and serving generated images and other attachments.

## Architecture

The platform consists of three main components:

* **Backend (Rust):**
  * Web server (Actix Web) handles API requests from the frontend.
  * Data access layer (Diesel ORM) interacts with the PostgreSQL database.
  * FluentCLI integration executes FluentCLI commands and manages job execution.
  * LLM service integrates multiple AI providers for chat functionality.
  * Attachment service manages file storage and retrieval.
  * Temporary file service handles ephemeral data like generated images.
* **Database (PostgreSQL):**
  * Stores user data, job configurations, execution history, and chat data.
  * Manages attachment metadata and file paths.
  * Handles user-specific LLM configurations and API keys.
* **Frontend (Tauri):**
  * User interface built with HTML, CSS, JavaScript, and Vue.js.
  * Tauri API enables communication with the backend and access to native desktop features.
  * Real-time chat interface with streaming responses.
  * Image gallery for viewing and managing generated images.

## Getting Started

### Prerequisites

* Docker
* Docker Compose
* API keys for desired LLM providers

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/fluentcli-web-services.git
   ```

2. Set up environment variables:
   * Run `./scripts/setup_env.sh` to create a `.env` file from `.env.example`
   * Edit `.env` to add your API keys and any custom paths

3. Build and start the application using Docker Compose:

   ```bash
   ./scripts/start_dev.sh
   ```

4. Access the application:
   * Desktop application: The Tauri application will be launched automatically.
   * Web application: Open your browser and navigate to `http://localhost:1420`.

## Development Environment

The `scripts` directory contains helper scripts for local development:

```bash
./scripts/setup_env.sh  # create the .env file
./scripts/start_dev.sh  # build and start Docker containers
```

After the containers start you can follow their logs with:

```bash
docker-compose logs -f
```

## Usage

1. **Create a user account:**
   * Use the API endpoint `/users` to create a new user account.
2. **Manage API keys:**
   * Use the API endpoints under `/users/{id}/api_keys` to manage your API keys.
3. **Configure Amber Store and Vault Store:**
   * Use the API endpoints for Amber Store and Vault Store to manage your secure data.
4. **Configure LLM providers:**
   * Set up provider-specific configurations in the LLM Provider Manager.
   * Configure API keys and model parameters for each provider.
5. **Create and manage configurations:**
   * Use the API endpoints under `/configurations` to create and manage configurations.
6. **Create and manage pipelines:**
   * Use the API endpoints under `/pipelines` to create and manage pipelines.
7. **Manage Docker files:**
   * Use the API endpoints under `/docker_files` to manage your Docker files.
8. **Create and manage jobs:**
   * Use the API endpoints under `/jobs` to create, manage, start, stop, and monitor jobs.
9. **Use the chat system:**
   * Use the API endpoints under `/chat` to create conversations, send messages, and manage attachments.
10. **Generate and manage images:**
    * Use image-capable LLM providers to generate images.
    * View and manage images in the Attachment Gallery.

## Chat System

The chat system provides the following functionality:

* Create and manage conversations
* Send and receive messages within conversations
* Attach files to messages
* Configure and use different LLM (Language Model) providers
* Manage user-specific LLM configurations
* Stream responses from AI models for real-time interaction
* Generate and display AI-generated images
* Manage attachments through a unified system

To use the chat system, refer to the API documentation for detailed information on the available endpoints and their usage.

## LLM Providers

FluentCLI Web Services supports multiple Language Model providers, each with its own implementation and configuration. The following providers are currently supported:

1. **GPT (OpenAI):**
   * Text generation using GPT-3.5 and GPT-4 models
   * Image generation through DALL-E integration
   * Streaming support for real-time responses

2. **Claude (Anthropic):**
   * Advanced language understanding and generation
   * Support for long context windows
   * Specialized knowledge domains

3. **Gemini (Google):**
   * State-of-the-art language capabilities
   * Multi-modal understanding
   * Efficient processing of large contexts

4. **Command (Cohere):**
   * Specialized command understanding
   * Custom model fine-tuning support
   * Enterprise-grade reliability

5. **DALL-E (OpenAI):**
   * High-quality image generation
   * Style and composition control
   * Multiple size options

6. **Perplexity:**
   * Advanced language processing
   * Real-time information updates
   * Citation and source tracking

7. **Leonardo AI:**
   * Advanced image generation capabilities
   * Style customization
   * Multiple aspect ratio support

8. **Stability AI:**
   * High-fidelity image generation
   * Fine-grained control over generation parameters
   * Multiple model support (SD-XL, SD-XL-Turbo)
9. **Mistral:**
   * Lightweight yet powerful open-source models
   * Fast inference and cost effective

Each provider is implemented as a separate module in the `src/services/llm_providers` directory, following the `LLMProviderTrait` interface which includes:

* Request preparation with provider-specific parameters
* Response parsing and processing
* Streaming support for real-time interaction
* Error handling and logging
* File and attachment management

## Image Generation

The platform includes robust support for AI image generation through multiple providers:

### Supported Providers

* **DALL-E:** OpenAI's image generation model
* **Leonardo AI:** Advanced image generation with style control
* **Stability AI:** High-quality image generation with fine-tuned control

### Image Processing Flow

1. **Generation:**
   * User submits prompt through chat interface
   * Request is sent to selected provider
   * Provider generates image and returns data

2. **Storage:**
   * Images are saved to the configured storage location
   * Metadata is stored in the database
   * Temporary files are managed automatically

3. **Serving:**
   * Images are served through dedicated endpoints
   * Support for various formats (PNG, JPEG, WebP)
   * Proper content type handling

4. **Management:**
   * View images in the Attachment Gallery
   * Download generated images
   * Delete unwanted images

### Configuration

Each image provider can be configured with:

* API keys and authentication
* Model selection
* Generation parameters
* Output format preferences
* Storage locations

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
* Image generation tests
* Attachment handling tests
* And more...

Ensure all tests pass before submitting pull requests or deploying to production.

## Metrics and Environment

The backend exposes a Prometheus-compatible metrics endpoint at `/metrics`.
Set the `APP_ENV` environment variable to `development` or `production` to
control environment-specific behavior. The current mode is printed on startup.

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
* The teams behind the various LLM providers for their powerful APIs

For any questions, issues, or feature requests, please open an issue on the GitHub repository or contact the maintainers directly.
