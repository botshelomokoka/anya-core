#!/bin/bash

# Configuration
ORG_NAME="anya"  # Replace with your organization name
GITHUB_TOKEN="${GITHUB_TOKEN}"  # Set this in your environment
GPG_KEY="6116504FE0507099"
GIT_NAME="bo_thebig"
GIT_EMAIL="botshelomokoka@gmail.com"

# Required files to copy to all repos
WORKFLOW_FILES=(
    ".github/workflows/release_flow.yml"
    ".github/workflows/branch_maintenance.yml"
    ".github/dependabot.yml"
)

# Get all repositories
echo "Fetching repositories..."
repos=$(gh repo list $ORG_NAME --json nameWithOwner --jq '.[].nameWithOwner')

for repo in $repos; do
    echo "Processing repository: $repo"
    
    # Clone repository
    git clone "https://github.com/$repo.git" temp_repo
    cd temp_repo
    
    # Setup git config with GPG
    git config user.name "$GIT_NAME"
    git config user.email "$GIT_EMAIL"
    git config user.signingkey "$GPG_KEY"
    git config commit.gpgsign true
    
    # Create development branch if it doesn't exist
    if ! git show-ref --verify --quiet refs/remotes/origin/development; then
        echo "Creating development branch for $repo"
        git checkout -b development main || git checkout -b development master
        git push origin development
    fi
    
    # Copy workflow files
    mkdir -p .github/workflows
    for file in "${WORKFLOW_FILES[@]}"; do
        cp "../$file" "./$file" 2>/dev/null || echo "Warning: Could not copy $file"
    done
    
    # Commit changes if any
    if git status --porcelain | grep -q '^'; then
        git add .
        git commit -m "chore: standardize repository structure and workflows"
        git push origin development
    fi
    
    # Analyze and clean branches
    echo "Analyzing branches for $repo"
    
    # List merged branches
    merged_branches=$(git branch --merged development | grep -v "^\*" | grep -vE "^(\s*development|\s*main|\s*master)$")
    
    # List stale branches (no commits in 30 days)
    stale_branches=$(git for-each-ref --sort=-committerdate refs/heads/ --format='%(refname:short)|%(committerdate:iso8601)' | 
        while IFS='|' read -r branch date; do
            if [[ ! "$branch" =~ ^(main|development|master)$ ]] && [[ $(date -d "$date" +%s) -lt $(date -d "30 days ago" +%s) ]]; then
                echo "$branch"
            fi
        done)
    
    # Generate report
    echo "# Branch Cleanup Report for $repo" > "../reports/${repo//\//_}_report.md"
    echo "## Merged Branches" >> "../reports/${repo//\//_}_report.md"
    echo "$merged_branches" >> "../reports/${repo//\//_}_report.md"
    echo "## Stale Branches" >> "../reports/${repo//\//_}_report.md"
    echo "$stale_branches" >> "../reports/${repo//\//_}_report.md"
    
    # Create issue with report
    gh issue create \
        --repo "$repo" \
        --title "Branch Cleanup Required" \
        --body-file "../reports/${repo//\//_}_report.md" \
        --label "maintenance,branch-cleanup"
    
    cd ..
    rm -rf temp_repo
done

echo "Repository standardization complete!"
