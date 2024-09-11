#!/bin/bash

# Output file
output_file="compiled_output.txt"

# Function to check if file has allowed extension
has_allowed_extension() {
    local file="$1"
    local allowed_extensions=(".md" "dockerfile*" ".yaml" ".rs" ".sh" ".ts" ".json" ".sql" ".vue")
    for ext in "${allowed_extensions[@]}"; do
        if [[ "$file" == *"$ext" ]]; then
            return 0  # File has an allowed extension
        fi
    done
    return 1  # File does not have an allowed extension
}

# Function to process files
process_files() {
    local dir="$1"
    local prefix="$2"
    
    # Check if directory is empty or doesn't exist
    if [ ! -d "$dir" ] || [ -z "$(ls -A "$dir")" ]; then
        return
    fi

    for file in "$dir"/*; do
        # Check if file exists (in case it was deleted during execution)
        if [ ! -e "$file" ]; then
            continue
        fi

        if [ -f "$file" ]; then
            # Check if file has an allowed extension
            if has_allowed_extension "$file"; then
                echo "==== START OF FILE: $prefix$(basename "$file") ====" >> "$output_file"
                cat "$file" >> "$output_file"
                echo "==== END OF FILE: $prefix$(basename "$file") ====" >> "$output_file"
                echo "" >> "$output_file"
            fi
        elif [ -d "$file" ]; then
            local new_prefix="$prefix$(basename "$file")/"
            if [ "$(basename "$file")" = "src" ] || [ "$prefix" = "" ] || [ "$prefix" = "frontend/" ]; then
                process_files "$file" "$new_prefix"
            fi
        fi
    done
}

# Clear or create the output file
> "$output_file"

# Process current directory
process_files "." ""

# Process frontend directory if it exists
if [ -d "frontend" ]; then
    process_files "frontend" "frontend/"
fi

echo "Compilation complete. Output saved to $output_file"