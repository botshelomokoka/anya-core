# PowerShell Script for Git Commit and Push
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# Configuration
$MAIN_BRANCH = "main"
$COMMIT_MSG_MIN_LENGTH = 10
$REQUIRED_GIT_CONFIGS = @("user.name", "user.email")

# Import common utilities
$SCRIPT_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path

function Write-Log {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Message,
        [ValidateSet("Info", "Warning", "Error")]
        [string]$Level = "Info"
    )
    
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $color = switch ($Level) {
        "Info" { "White" }
        "Warning" { "Yellow" }
        "Error" { "Red" }
    }
    Write-Host "[$timestamp] $Message" -ForegroundColor $color
}

function Check-GitConfig {
    Write-Log "Checking git configuration..." -Level Info
    $missingConfigs = @()
    
    foreach ($config in $REQUIRED_GIT_CONFIGS) {
        try {
            $null = git config --get $config
        } catch {
            $missingConfigs += $config
        }
    }
    
    if ($missingConfigs.Count -gt 0) {
        Write-Log "Missing git configurations: $($missingConfigs -join ', ')" -Level Error
        Write-Log "Please set them using:" -Level Info
        foreach ($config in $missingConfigs) {
            Write-Host "git config --global $config 'YOUR_$($config.ToUpper())'"
        }
        exit 1
    }
}

function Test-CommitMessage {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Message
    )
    
    # Check minimum length
    if ($Message.Length -lt $COMMIT_MSG_MIN_LENGTH) {
        Write-Log "Commit message too short. Minimum length is $COMMIT_MSG_MIN_LENGTH characters." -Level Error
        return $false
    }
    
    # Check conventional commit format
    $pattern = "^(feat|fix|docs|style|refactor|test|chore)(\([a-z]+\))?: .+$"
    if ($Message -notmatch $pattern) {
        Write-Log "Invalid commit message format. Please use conventional commits:" -Level Error
        Write-Log "type(scope): description" -Level Info
        Write-Log "Types: feat, fix, docs, style, refactor, test, chore" -Level Info
        return $false
    }
    
    return $true
}

function Check-Branch {
    $currentBranch = git rev-parse --abbrev-ref HEAD
    
    if ($currentBranch -ne $MAIN_BRANCH) {
        Write-Log "You are not on $MAIN_BRANCH branch (current: $currentBranch)" -Level Warning
        $response = Read-Host "Do you want to continue? [y/N]"
        if ($response -notmatch "^[Yy]$") {
            exit 1
        }
    }
}

function Invoke-CommitChanges {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Message
    )
    
    # Check if there are changes to commit
    $status = git status --porcelain
    if ($status) {
        git add .
        if (Test-CommitMessage $Message) {
            git commit -m $Message
        } else {
            exit 1
        }
    } else {
        Write-Log "No changes to commit" -Level Warning
        exit 0
    }
}

function Push-Changes {
    $currentBranch = git rev-parse --abbrev-ref HEAD
    
    # Fetch and check for conflicts
    git fetch origin
    try {
        git merge-base --is-ancestor origin/$currentBranch HEAD
    } catch {
        Write-Log "Remote branch has diverged. Please pull changes first:" -Level Error
        Write-Log "git pull origin $currentBranch --rebase" -Level Info
        exit 1
    }
    
    git push origin $currentBranch
}

function Main {
    # Ensure we're in a git repository
    try {
        $null = git rev-parse --git-dir
    } catch {
        Write-Log "Not a git repository" -Level Error
        exit 1
    }
    
    Check-GitConfig
    Check-Branch
    
    # Get commit message from args or prompt
    $commitMessage = if ($args.Count -gt 0) {
        $args[0]
    } else {
        Read-Host "Enter commit message"
    }
    
    Invoke-CommitChanges $commitMessage
    Push-Changes
    
    Write-Log "Successfully committed and pushed changes" -Level Info
}

# Run main if script is executed directly
if ($MyInvocation.InvocationName -eq $MyInvocation.MyCommand.Path) {
    Main $args
}
