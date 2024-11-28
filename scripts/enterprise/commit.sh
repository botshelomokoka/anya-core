#!/bin/bash

# Enterprise commit script for Anya

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../common/utils.sh
source "$SCRIPT_DIR/common/utils.sh"

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

commit_enterprise() {
    local project_root
    project_root=$(get_project_root)
    
    # Check git configuration
    check_git_config
    
    # Check if we're in the enterprise directory
    if [ ! -d "$project_root/enterprise" ]; then
        error "Enterprise directory not found"
    fi
    
    cd "$project_root/enterprise" || error "Failed to change to enterprise directory"
    
    # Initialize repository if needed
    if [ ! -d ".git" ]; then
        log "Initializing enterprise repository..."
        git init
        git add .
        git commit -m "feat(enterprise): initial commit"
    fi
    
    # Check if there are changes to commit
    if git diff --quiet && git diff --staged --quiet; then
        log "No changes to commit in enterprise"
        return 0
    fi
    
    # Stage all changes
    git add .
    
    # Create commit message with structured format
    local date_str
    date_str=$(date +"%Y-%m-%d")
    
    # Get the list of changed files
    local changed_files
    changed_files=$(git diff --cached --name-only)
    
    # Determine primary change category
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
    
    # Create a more detailed commit message
    local commit_msg="$category(enterprise): comprehensive module update [$date_str]

Changes Overview:
- Updated enterprise configuration and setup
- Enhanced security and compliance features
- Improved ML integration and analytics
- Added comprehensive testing

Modified Components:
$(git diff --cached --stat | sed 's/^/- /')

Impact:
- Module: enterprise
- Type: major update
- Date: $date_str
- Changes: $(git diff --cached --shortstat)"
    
    # Commit changes
    git commit -m "$commit_msg"
    
    # Update parent repository if this is a submodule
    cd "$project_root" || error "Failed to return to project root"
    if git submodule status enterprise &>/dev/null; then
        log "Updating enterprise submodule reference..."
        git add enterprise
        git commit -m "chore: update enterprise submodule reference [$date_str]"
    fi
    
    # Push changes if remote exists
    if git remote get-url origin &>/dev/null; then
        log "Pushing changes to remote..."
        git push origin HEAD || error "Failed to push changes"
    fi
    
    log "Enterprise changes committed successfully"
}

# Run commit if script is executed directly
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    commit_enterprise
fi
