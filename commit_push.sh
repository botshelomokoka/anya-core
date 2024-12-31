#!/bin/bash
set -euo pipefail

# Import common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/scripts/lib/common.sh"

# Configuration
readonly MAIN_BRANCH="main"
readonly COMMIT_MSG_MIN_LENGTH=10
readonly REQUIRED_GIT_CONFIGS=("user.name" "user.email")

check_git_config() {
    log_info "Checking git configuration..."
    local missing_configs=()
    
    for config in "${REQUIRED_GIT_CONFIGS[@]}"; do
        if ! git config --get "$config" &>/dev/null; then
            missing_configs+=("$config")
        fi
    done
    
    if (( ${#missing_configs[@]} > 0 )); then
        log_error "Missing git configurations: ${missing_configs[*]}"
        log_info "Please set them using:"
        for config in "${missing_configs[@]}"; do
            echo "git config --global $config \"YOUR_${config^^}\""
        done
        exit 1
    fi
}

validate_commit_message() {
    local message=$1
    
    # Check minimum length
    if (( ${#message} < COMMIT_MSG_MIN_LENGTH )); then
        log_error "Commit message too short. Minimum length is $COMMIT_MSG_MIN_LENGTH characters."
        exit 1
    fi
    
    # Check conventional commit format
    if ! echo "$message" | grep -qE "^(feat|fix|docs|style|refactor|test|chore)(\([a-z]+\))?: .+$"; then
        log_error "Invalid commit message format. Please use conventional commits:"
        log_info "type(scope): description"
        log_info "Types: feat, fix, docs, style, refactor, test, chore"
        exit 1
    fi
}

check_branch() {
    local current_branch
    current_branch=$(git rev-parse --abbrev-ref HEAD)
    
    if [[ "$current_branch" != "$MAIN_BRANCH" ]]; then
        log_warn "You are not on $MAIN_BRANCH branch (current: $current_branch)"
        read -rp "Do you want to continue? [y/N] " response
        if [[ ! "$response" =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

commit_changes() {
    local message=$1
    
    # Check if there are changes to commit
    if ! git diff --quiet || ! git diff --cached --quiet; then
        git add .github/workflows/update-roadmap.yml scripts/update-roadmap.js package.json .github/ISSUE_TEMPLATE/roadmap_item.md
        validate_commit_message "$message"
        git commit -m "$message"
    else
        log_warn "No changes to commit"
        exit 0
    fi
}

push_changes() {
    local current_branch
    current_branch=$(git rev-parse --abbrev-ref HEAD)
    
    # Fetch and check for conflicts
    git fetch origin
    if ! git merge-base --is-ancestor origin/"$current_branch" HEAD; then
        log_error "Remote branch has diverged. Please pull changes first:"
        log_info "git pull origin $current_branch --rebase"
        exit 1
    fi
    
    git push origin "$current_branch"
}

main() {
    # Ensure we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        log_error "Not a git repository"
        exit 1
    fi
    
    check_git_config
    check_branch
    
    # Get commit message from args or prompt
    local commit_message
    if [[ $# -gt 0 ]]; then
        commit_message="$1"
    else
        read -rp "Enter commit message: " commit_message
    fi
    
    commit_changes "$commit_message"
    push_changes
    
    log_info "Successfully committed and pushed changes"
}

# Run main if script is executed directly
if [[ "${BASH_SOURCE[0]}" = "$0" ]]; then
    main "$@"
fi