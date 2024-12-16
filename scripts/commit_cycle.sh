#!/bin/bash

# Commit cycle script for Anya project
# Usage: ./commit_cycle.sh <commit_message>

set -e  # Exit on error

if [ -z "$1" ]; then
    echo "Error: Please provide a commit message"
    echo "Usage: ./commit_cycle.sh <commit_message>"
    exit 1
fi

COMMIT_MSG="$1"
ROOT_DIR=$(pwd)
SUBMODULES=("dash33" "dependencies" "enterprise")

echo "🔄 Starting commit cycle..."

# Function to commit changes in a repository
commit_repo() {
    local repo_path=$1
    local repo_name=$2
    local message=$3

    echo "Processing $repo_name..."
    cd "$repo_path"

    # Pull latest changes
    git pull origin main || { echo "Failed to pull $repo_name"; exit 1; }

    # Add and commit changes
    git add . || { echo "Failed to stage changes in $repo_name"; exit 1; }
    
    # Only commit if there are changes
    if ! git diff --cached --quiet; then
        git commit -m "$message" || { echo "Failed to commit $repo_name"; exit 1; }
        git push origin main || { echo "Failed to push $repo_name"; exit 1; }
        echo "✅ $repo_name: Changes committed and pushed"
    else
        echo "ℹ️ $repo_name: No changes to commit"
    fi
}

# 1. Process submodules first
for submodule in "${SUBMODULES[@]}"; do
    commit_repo "$ROOT_DIR/$submodule" "$submodule" "feat($submodule): $COMMIT_MSG"
done

# 2. Update main repository submodule references
cd "$ROOT_DIR"
git add "${SUBMODULES[@]}" || { echo "Failed to update submodule references"; exit 1; }
git commit -m "chore: Update submodule references" || true

# 3. Commit main repository changes
commit_repo "$ROOT_DIR" "main" "feat: $COMMIT_MSG"

echo "✅ Commit cycle completed successfully"
