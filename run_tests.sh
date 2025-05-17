#!/bin/bash
set -e

echo "Running all test scripts..."

# Make sure all test scripts are executable
chmod +x test_*.sh

# Run all test scripts
for test_script in test_*.sh; do
    echo "Running $test_script..."
    ./$test_script
    echo "$test_script completed."
    echo
done

echo "All tests completed successfully."
