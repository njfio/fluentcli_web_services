#!/bin/bash
# This script finds and replaces any hardcoded references to port 8001 in the built JavaScript files

echo "Fixing API URL in frontend files..."

# Find all JavaScript files in the assets directory
JS_FILES=$(find /usr/share/nginx/html/assets -name "*.js")

# Replace any references to port 8001 with port 8000
for file in $JS_FILES; do
  echo "Checking file: $file"
  if grep -q "localhost:8001" "$file"; then
    echo "Found reference to localhost:8001 in $file, replacing..."
    sed -i 's/localhost:8001/localhost/g' "$file"
    echo "Replacement complete."
  fi
done

echo "API URL fix complete."
