#!/bin/bash

# Function to create directory and README
create_dir_with_readme() {
    local dir=$1
    local title=$2
    mkdir -p "$dir"
    echo "# $title" > "$dir/README.md"
    echo "" >> "$dir/README.md"
    echo "Documentation for $title" >> "$dir/README.md"
}

# Function to process SUMMARY.md and create structure
process_summary() {
    local base_dir=$1
    local summary_file="$base_dir/SUMMARY.md"
    
    # Extract markdown links and create directories/files
    while IFS= read -r line; do
        if [[ $line =~ \[([^\]]+)\]\(([^\)]+)\) ]]; then
            local title="${BASH_REMATCH[1]}"
            local path="${BASH_REMATCH[2]}"
            
            # Skip if it's just README.md or other root files
            if [[ $path == *"/"* ]]; then
                local dir_path="$base_dir/$(dirname "$path")"
                local file_path="$base_dir/$path"
                
                # Create directory if it doesn't exist
                mkdir -p "$dir_path"
                
                # Create markdown file if it doesn't exist
                if [ ! -f "$file_path" ]; then
                    echo "# $title" > "$file_path"
                    echo "" >> "$file_path"
                    echo "Documentation for $title" >> "$file_path"
                fi
            fi
        fi
    done < "$summary_file"
}

# Main documentation directories
DOCS_DIRS=(
    "docs"
    "dependencies/docs"
    "anya-bitcoin/docs"
    "anya-enterprise/docs"
    "anya-extensions/docs"
)

# Process each documentation directory
for dir in "${DOCS_DIRS[@]}"; do
    if [ -f "$dir/SUMMARY.md" ]; then
        echo "Processing $dir..."
        process_summary "$dir"
    fi
done

echo "Documentation structure generated successfully!"
