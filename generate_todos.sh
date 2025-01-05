#!/bin/bash

# Define the root directory and files
ROOT_DIR="/home/portiam/Downloads/OPSource/anya"
TODO_FILE="${ROOT_DIR}/TODO.md"
SUMMARY_FILE="${ROOT_DIR}/todo_summary.md"

# First, create our main TODO.md file
cat > "$TODO_FILE" << 'TODOEOF'
# Anya Project TODOs and Implementation Status

## Current Status (as of 2025-01-04)

### 1. Dependency Management
- [x] Initial dependency conflict identification
- [ ] Automated version resolution system
- [ ] Integration with Docker-based development environment

### 2. GitHub Workflow Updates
- [x] Updated ai-review.yml with correct action versions
- [x] Fixed CodeQL analysis parameters
- [x] Corrected performance check action version

### 3. System Compatibility
- [ ] Implement comprehensive system checks
- [ ] Add Dart SDK version verification
- [ ] Document system requirements

### 4. Known Issues
1. Dependency Conflicts:
   - http ^1.2.0 vs dart_code_metrics requirements
   - web5 ^0.4.0 requiring specific http version
   - mockito version compatibility issues

### 5. Next Actions
- [ ] Resolve remaining dependency conflicts
- [ ] Complete system compatibility checks
- [ ] Test file management scripts
- [ ] Document all changes
- [ ] Update version history
- [ ] Implement automated version resolution
- [ ] Create comprehensive testing suite

Last Updated: 2025-01-04
TODOEOF

# Now create the summary file
cat > "$SUMMARY_FILE" << SUMEOF
# TODO Files Summary
Generated on: $(date)

## Found TODO Files
SUMEOF

# Function to process TODO.md files
process_todo_file() {
    local file="$1"
    local relative_path="${file#$ROOT_DIR/}"
    
    echo -e "\n### $relative_path" >> "$SUMMARY_FILE"
    echo '```markdown' >> "$SUMMARY_FILE"
    cat "$file" >> "$SUMMARY_FILE"
    echo '```' >> "$SUMMARY_FILE"
}

# Find all TODO.md files (case insensitive)
echo "Searching for TODO files..."
while IFS= read -r -d '' file; do
    process_todo_file "$file"
done < <(find "$ROOT_DIR" -type f -iname "TODO*.md" -print0)

# Add a summary section
echo -e "\n## Summary of TODOs" >> "$SUMMARY_FILE"
echo "Total TODO files found: $(grep -c "^### " "$SUMMARY_FILE")" >> "$SUMMARY_FILE"

echo "Search completed. Results written to $SUMMARY_FILE"
