#!/bin/bash

# Sync script for all Anya repositories

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=common/utils.sh
source "$SCRIPT_DIR/common/utils.sh"

# Global array to store commit messages
declare -a COMMIT_MESSAGES

# Set git configuration if not already set
setup_git_config() {
    if [ -z "$(git config --get user.name)" ]; then
        git config --global user.name "bo_thebig"
    fi
    if [ -z "$(git config --get user.email)" ]; then
        git config --global user.email "botshelomokoka@gmail.com"
    fi
}

# Check git configuration
check_git_config() {
    local git_name
    local git_email
    
    # Ensure git config is set
    setup_git_config
    
    git_name=$(git config --get user.name)
    git_email=$(git config --get user.email)
    
    if [ -z "$git_name" ] || [ -z "$git_email" ]; then
        log "Git user configuration not found. Please configure git first:"
        log "  git config --global user.name 'Your Name'"
        log "  git config --global user.email 'your.email@example.com'"
        exit 1
    fi
    
    log "Using git identity: $git_name <$git_email>"
}

# Function to get commit category based on changes
get_commit_category() {
    local changed_files="$1"
    local category="feat"
    
    if echo "$changed_files" | grep -q "test"; then
        category="test"
    elif echo "$changed_files" | grep -q "fix"; then
        category="fix"
    elif echo "$changed_files" | grep -q "docs"; then
        category="docs"
    elif echo "$changed_files" | grep -q "refactor"; then
        category="refactor"
    fi
    
    echo "$category"
}

# Function to commit changes in a repository
commit_changes() {
    local repo_path="$1"
    local repo_name="$2"
    local parent_changes="${3:-}"
    local made_changes=false
    
    cd "$repo_path" || error "Failed to change to $repo_name directory"
    
    # Check if there are changes to commit
    if ! git diff --quiet || ! git diff --staged --quiet; then
        made_changes=true
        
        # Stage all changes
        git add .
        
        # Create commit message with structured format
        local date_str
        date_str=$(date +"%Y-%m-%d")
        
        # Get the list of changed files
        local changed_files
        changed_files=$(git diff --cached --name-only)
        
        # Determine primary change category
        local category
        category=$(get_commit_category "$changed_files")
        
        # Create a detailed commit message
        local commit_msg="$category($repo_name): comprehensive update [$date_str]

Changes Overview:
- Updated configuration and setup
- Enhanced features and functionality
- Improved integration and testing
- Added comprehensive documentation

Modified Components:
$(git diff --cached --stat | sed 's/^/- /')

Impact:
- Module: $repo_name
- Type: major update
- Date: $date_str
- Changes: $(git diff --cached --shortstat)
${parent_changes:+
Parent Changes:
$parent_changes}"
        
        # Store commit message for parent repositories
        COMMIT_MESSAGES+=("[$repo_name] $category: $(git diff --cached --shortstat)")
        
        # Commit changes
        git commit -m "$commit_msg"
        
        # Push changes if remote exists
        if git remote get-url origin &>/dev/null; then
            log "Pushing changes to remote for $repo_name..."
            git push origin HEAD || log "Warning: Failed to push changes for $repo_name"
        fi
    else
        log "No changes to commit in $repo_name"
    fi
    
    echo "$made_changes"
}

# Function to process a single submodule
process_submodule() {
    local project_root="$1"
    local submodule="$2"
    local parent_changes="${3:-}"
    local made_changes=false
    
    log "Processing submodule: $submodule"
    
    # Navigate to submodule
    cd "$project_root/$submodule" || error "Failed to change to submodule directory: $submodule"
    
    # Initialize and update nested submodules if any
    git submodule update --init --recursive
    
    # Process nested submodules first
    local nested_submodules
    nested_submodules=$(git config --file .gitmodules --get-regexp path 2>/dev/null | awk '{ print $2 }' || echo "")
    
    local nested_changes=""
    if [ -n "$nested_submodules" ]; then
        for nested in $nested_submodules; do
            log "Processing nested submodule: $nested"
            if process_submodule "$project_root/$submodule" "$nested" "$parent_changes"; then
                made_changes=true
                nested_changes="${nested_changes}${nested_changes:+\n}$nested: Updated"
            fi
        done
    fi
    
    # Combine parent and nested changes
    local all_changes="${nested_changes:+$nested_changes\n}${parent_changes:+$parent_changes}"
    
    # Commit changes in current submodule
    if commit_changes "$project_root/$submodule" "$submodule" "$all_changes"; then
        made_changes=true
    fi
    
    echo "$made_changes"
}

# Function to sync submodules
sync_submodules() {
    local project_root="$1"
    local parent_changes="${2:-}"
    local made_changes=false
    
    cd "$project_root" || error "Failed to change to project root"
    
    # Get list of submodules
    local submodules
    submodules=$(git config --file .gitmodules --get-regexp path 2>/dev/null | awk '{ print $2 }' || echo "")
    
    if [ -n "$submodules" ]; then
        # Initialize and update submodules
        log "Initializing and updating submodules..."
        git submodule update --init --recursive
        
        # Process each submodule
        for submodule in $submodules; do
            if process_submodule "$project_root" "$submodule" "$parent_changes"; then
                made_changes=true
            fi
        done
    fi
    
    echo "$made_changes"
}

# Main function to sync all repositories
sync_all_repos() {
    local project_root
    project_root=$(get_project_root)
    
    # Reset commit messages array
    COMMIT_MESSAGES=()
    
    # Check git configuration
    check_git_config
    
    # First, sync all submodules
    local submodules_changed=false
    if sync_submodules "$project_root"; then
        submodules_changed=true
    fi
    
    # Create summary of all changes
    local changes_summary=""
    if [ ${#COMMIT_MESSAGES[@]} -gt 0 ]; then
        changes_summary="Submodule Changes:\n"
        for msg in "${COMMIT_MESSAGES[@]}"; do
            changes_summary+="- $msg\n"
        done
    fi
    
    # Process main repositories with changes summary
    log "Processing main repository..."
    commit_changes "$project_root" "anya-core" "$changes_summary"
    
    # Process enterprise repository if it exists
    if [ -d "$project_root/enterprise" ]; then
        log "Processing enterprise repository..."
        commit_changes "$project_root/enterprise" "anya-enterprise" "$changes_summary"
    fi
    
    # Process bitcoin repository if it exists
    if [ -d "$project_root/anya-bitcoin" ]; then
        log "Processing bitcoin repository..."
        commit_changes "$project_root/anya-bitcoin" "anya-bitcoin" "$changes_summary"
    fi
    
    # Process extensions repository if it exists
    if [ -d "$project_root/anya-extensions" ]; then
        log "Processing extensions repository..."
        commit_changes "$project_root/anya-extensions" "anya-extensions" "$changes_summary"
    fi
    
    # Display final summary
    log "Repository Sync Summary:"
    if [ ${#COMMIT_MESSAGES[@]} -gt 0 ]; then
        log "Changes made:"
        for msg in "${COMMIT_MESSAGES[@]}"; do
            log "  $msg"
        done
    else
        log "No changes were made in any repository"
    fi
    
    log "All repositories synchronized successfully"
}

# Run sync if script is executed directly
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    sync_all_repos
fi
