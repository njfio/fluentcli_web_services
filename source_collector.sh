#!/bin/bash

# Output file
output_file="compiled_output.txt"

# Allowed and not allowed extensions
allowed_extensions=(".md" "dockerfile*" ".yaml" ".rs" ".sh" ".ts" ".json" ".sql" ".vue" ".toml")
not_allowed_extensions=(".log" ".tmp" ".bak" ".swp" "*lock*" ".env")

# Not allowed folder names
not_allowed_folders=("node_modules" "target" "dist" "build" ".git")

# Function to check if file has allowed extension
has_allowed_extension() {
    local file="$1"
    for ext in "${allowed_extensions[@]}"; do
        if [[ "$file" == *"$ext" ]]; then
            return 0  # File has an allowed extension
        fi
    done
    return 1  # File does not have an allowed extension
}

# Function to check if file has not allowed extension
has_not_allowed_extension() {
    local file="$1"
    for ext in "${not_allowed_extensions[@]}"; do
        if [[ "$file" == *"$ext" ]]; then
            return 0  # File has a not allowed extension
        fi
    done
    return 1  # File does not have a not allowed extension
}

# Function to check if path contains not allowed folder
has_not_allowed_folder() {
    local path="$1"
    for folder in "${not_allowed_folders[@]}"; do
        if [[ "$path" == *"/$folder/"* || "$path" == *"/$folder" ]]; then
            return 0  # Path contains a not allowed folder
        fi
    done
    return 1  # Path does not contain a not allowed folder
}

# Function to process files
process_files() {
    local dir="$1"
    local prefix="$2"
    
    # Use find to recursively search for files
    find "$dir" -type f | while read -r file; do
        # Get the relative path of the file
        local rel_path="${file#$dir}"
        
        # Check if file path contains not allowed folder
        if has_not_allowed_folder "$file"; then
            continue
        fi
        
        # Check if file has an allowed extension and doesn't have a not allowed extension
        if has_allowed_extension "$file" && ! has_not_allowed_extension "$file"; then
            echo "==== START OF FILE: $prefix$rel_path ====" >> "$output_file"
            cat "$file" >> "$output_file"
            echo "==== END OF FILE: $prefix$rel_path ====" >> "$output_file"
            echo "" >> "$output_file"
        fi
    done
}

# Clear or create the output file
> "$output_file"

# Process src directory if it exists
if [ -d "./src" ]; then
    process_files "./src" "src/"
fi

# Process frontend/src directory if it exists
if [ -d "./frontend/src" ]; then
    process_files "./frontend/src" "frontend/src/"
fi

echo "Compilation complete. Output saved to $output_file"