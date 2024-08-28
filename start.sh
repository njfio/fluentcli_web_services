#!/bin/bash
set -e

# Wait for the database to be ready
until PGPASSWORD=$POSTGRES_PASSWORD psql -h "db" -U "user" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up - executing command"

# Run Diesel setup and migrations
diesel setup
diesel migration run

# Start the application
exec /app/fws