#!/bin/bash
set -e

echo "Setting up development environment for Codex Service (Rust & Front-end)..."
echo "This script will install all necessary dependencies and configure the environment."

# Check for required tools
echo "Checking for required tools..."
rustc --version || echo "Rust not found, but will use pre-installed version in the container"
node --version || echo "Node.js not found, but will use pre-installed version in the container"
npm --version || echo "npm not found, but will use pre-installed version in the container"

# Configure Rust components (using pre-installed Rust)
echo "Configuring Rust components..."
if command -v rustup &> /dev/null; then
    # Add Rust components that are likely to be needed
    rustup component add rustfmt clippy 2>/dev/null || echo "Failed to add Rust components, but continuing..."
    rustup target add wasm32-unknown-unknown 2>/dev/null || echo "Failed to add wasm target, but continuing..."
fi

# Set up project structure
echo "Setting up project structure..."
mkdir -p ./src/{bin,lib,models,services,utils,web}
mkdir -p ./frontend/{src,public,assets}
mkdir -p ./config
mkdir -p ./tests/{unit,integration}
mkdir -p ./docs

# Create basic configuration files
echo "Creating basic configuration files..."

# Create a basic Cargo.toml
cat > ./Cargo.toml << EOF
[package]
name = "fluent_web_services"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "Fluent Web Services"

[dependencies]
# Web frameworks
actix-web = "4.3"
tokio = { version = "1", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database
sqlx = { version = "0.6", features = ["runtime-tokio-rustls", "postgres"] }

# Utilities
anyhow = "1.0"
thiserror = "1.0"
log = "0.4"
env_logger = "0.10"
dotenv = "0.15"

[dev-dependencies]
mockall = "0.11"
tokio-test = "0.4"

[[bin]]
name = "server"
path = "src/bin/server.rs"
EOF

# Create a basic package.json for frontend
cat > ./frontend/package.json << EOF
{
  "name": "fluent-web-frontend",
  "version": "0.1.0",
  "private": true,
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.10.0",
    "axios": "^1.3.5"
  },
  "devDependencies": {
    "vite": "^4.2.1",
    "@vitejs/plugin-react": "^3.1.0",
    "eslint": "^8.38.0",
    "prettier": "^2.8.7"
  },
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  }
}
EOF

# Create a basic server.rs file
cat > ./src/bin/server.rs << EOF
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Starting Fluent Web Services server...");

    // TODO: Initialize server configuration

    // TODO: Set up database connection

    // TODO: Configure and start web server

    println!("Server is running!");

    // Keep the server running
    tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
    println!("Shutting down...");

    Ok(())
}
EOF

# Create a basic .gitignore file
cat > ./.gitignore << EOF
/target
/dist
/node_modules
**/*.rs.bk
Cargo.lock
**/.DS_Store
.env
.env.local
.env.development.local
.env.test.local
.env.production.local
npm-debug.log*
yarn-debug.log*
yarn-error.log*
EOF

# Create a basic README.md
cat > ./README.md << EOF
# Fluent Web Services

A web service built with Rust and modern front-end technologies.

## Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js (v18+)
- npm or yarn

### Backend Setup

\`\`\`bash
# Build the project
cargo build

# Run the server
cargo run --bin server
\`\`\`

### Frontend Setup

\`\`\`bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
\`\`\`

## Project Structure

- \`/src\` - Rust backend code
- \`/frontend\` - Frontend application
- \`/config\` - Configuration files
- \`/tests\` - Test files
- \`/docs\` - Documentation

## Development

### Running Tests

\`\`\`bash
cargo test
\`\`\`

### Building for Production

\`\`\`bash
# Build backend
cargo build --release

# Build frontend
cd frontend && npm run build
\`\`\`
EOF

# Create a basic .env file
cat > ./.env.example << EOF
# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
LOG_LEVEL=info

# Database Configuration
DATABASE_URL=postgres://username:password@localhost/database_name

# Frontend Configuration
VITE_API_URL=http://localhost:8080/api
EOF

echo "Setup completed successfully!"
echo "Project structure and configuration files have been created."
echo "You can now start developing your Fluent Web Services application."
