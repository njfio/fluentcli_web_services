#!/bin/bash
set -e

# Create the .env file for development if it does not exist.

if [ -f .env ]; then
  echo ".env already exists. Edit this file to update environment variables."
  exit 0
fi

if [ ! -f .env.example ]; then
  echo ".env.example not found. Cannot create .env."
  exit 1
fi

cp .env.example .env

cat <<EOM
Created .env from .env.example.
Please edit .env and provide values for:
  - API keys for your LLM providers
  - JWT_SECRET and ENCRYPTION_KEY
  - Paths such as UPLOAD_DIR
EOM

