# Fluent Web Services

A web service built with Rust and modern front-end technologies.

## Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js (v18+)
- npm or yarn

### Backend Setup

```bash
# Build the project
cargo build

# Run the server
cargo run --bin server
```

### Frontend Setup

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

## Project Structure

- `/src` - Rust backend code
- `/frontend` - Frontend application
- `/config` - Configuration files
- `/tests` - Test files
- `/docs` - Documentation

## Development

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
# Build backend
cargo build --release

# Build frontend
cd frontend && npm run build
```
