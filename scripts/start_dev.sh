#!/bin/bash
set -e

# Build and start the development environment using Docker Compose.

docker-compose pull

docker-compose build

docker-compose up -d

echo "FluentCLI Web Services is starting. Use 'docker-compose logs -f' to follow logs."

