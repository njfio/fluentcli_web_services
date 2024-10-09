#!/bin/bash
set -e

# Parse command-line arguments
RUN_TESTS=false
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --run-tests) RUN_TESTS=true ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

# Wait for the database to be ready
until PGPASSWORD=$POSTGRES_PASSWORD psql -h "db" -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up - executing command"

# Run migrations
#diesel migration run

# Run tests if specified
if [ "$RUN_TESTS" = true ] ; then
    echo "Running tests..."
    ./run_tests.sh
    echo "Tests completed."
fi

# Start the application
exec "/app/fws"