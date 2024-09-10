# Anya Core Project Manager

$configFile = "anya-core-config.json"

# Function to load or create configuration
function Get-Configuration {
    if (Test-Path $configFile) {
        $config = Get-Content $configFile | ConvertFrom-Json
    } else {
        $config = @{
            githubUser = ""
            githubToken = ""
            repoName = ""
        }
    }

    if ([string]::IsNullOrWhiteSpace($config.githubUser) -or 
        [string]::IsNullOrWhiteSpace($config.githubToken) -or 
        [string]::IsNullOrWhiteSpace($config.repoName)) {
        
        Write-Host "GitHub configuration is incomplete. Please provide the following details:" -ForegroundColor Yellow
        
        if ([string]::IsNullOrWhiteSpace($config.githubUser)) {
            $config.githubUser = Read-Host "Enter your GitHub username"
        }
        
        if ([string]::IsNullOrWhiteSpace($config.githubToken)) {
            $config.githubToken = Read-Host "Enter your GitHub personal access token" -AsSecureString
            $BSTR = [System.Runtime.InteropServices.Marshal]::SecureStringToBSTR($config.githubToken)
            $config.githubToken = [System.Runtime.InteropServices.Marshal]::PtrToStringAuto($BSTR)
        }
        
        if ([string]::IsNullOrWhiteSpace($config.repoName)) {
            $config.repoName = Read-Host "Enter the repository name (e.g., anya-core)"
        }

        $config | ConvertTo-Json | Set-Content $configFile
        Write-Host "Configuration saved." -ForegroundColor Green
    }

    return $config
}

$config = Get-Configuration

# Function to check if we're in a Git repository
function Test-GitRepository {
    if (-not (Test-Path .git)) {
        Write-Host "Error: This is not a Git repository." -ForegroundColor Red
        return $false
    }
    return $true
}

# Function to get all files recursively
function Get-AllFiles {
    Get-ChildItem -Recurse -File | Where-Object { $_.FullName -notlike "*\.git\*" }
}

# Function to get Cargo.toml information
function Get-CargoTomlInfo {
    if (Test-Path "Cargo.toml") {
        $content = Get-Content "Cargo.toml" -Raw
        $name = [regex]::Match($content, 'name\s*=\s*"(.+?)"').Groups[1].Value
        $version = [regex]::Match($content, 'version\s*=\s*"(.+?)"').Groups[1].Value
        $dependencies = [regex]::Matches($content, '\[dependencies\]([\s\S]*?)(\[|\z)') | ForEach-Object { $_.Groups[1].Value.Trim() }
        
        return @{
            Name = $name
            Version = $version
            Dependencies = $dependencies
        }
    }
    return $null
}

# Function to analyze Rust files
function Analyze-RustFiles {
    $rustFiles = Get-ChildItem -Recurse -Include *.rs
    $modules = @()
    $traits = @()
    $structs = @()

    foreach ($file in $rustFiles) {
        $content = Get-Content $file.FullName -Raw
        $modules += [regex]::Matches($content, 'mod\s+(\w+)') | ForEach-Object { $_.Groups[1].Value }
        $traits += [regex]::Matches($content, 'trait\s+(\w+)') | ForEach-Object { $_.Groups[1].Value }
        $structs += [regex]::Matches($content, 'struct\s+(\w+)') | ForEach-Object { $_.Groups[1].Value }
    }

    return @{
        Modules = $modules | Select-Object -Unique
        Traits = $traits | Select-Object -Unique
        Structs = $structs | Select-Object -Unique
    }
}

# Function to get Git status
function Get-GitStatus {
    $status = git status --porcelain
    $branchName = git rev-parse --abbrev-ref HEAD
    $lastCommit = git log -1 --pretty=format:"%h - %an, %ar : %s"

    return @{
        Status = if ($status) { $status } else { "Clean" }
        Branch = $branchName
        LastCommit = $lastCommit
    }
}

# Function to set up the project environment
function Setup-Environment {
    Write-Host "Setting up Anya Core environment..." -ForegroundColor Cyan

    # Install Rust if not already installed
    if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
        Write-Host "Installing Rust..." -ForegroundColor Yellow
        Invoke-WebRequest https://win.rustup.rs -OutFile rustup-init.exe
        .\rustup-init.exe -y
        Remove-Item rustup-init.exe
    }

    # Install additional dependencies
    Write-Host "Installing additional dependencies..." -ForegroundColor Yellow
    cargo install cargo-watch cargo-audit cargo-outdated

    # Set up project structure
    $directories = @("src", "tests", "docs", "scripts")
    foreach ($dir in $directories) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir | Out-Null
            Write-Host "Created directory: $dir" -ForegroundColor Green
        }
    }

    # Initialize Cargo.toml if it doesn't exist
    if (-not (Test-Path "Cargo.toml")) {
        cargo init --name $config.repoName
        Write-Host "Initialized Cargo.toml" -ForegroundColor Green
    }

    Write-Host "Environment setup complete." -ForegroundColor Green
}

# Function to run tests
function Run-Tests {
    Write-Host "Running tests..." -ForegroundColor Cyan
    cargo test
    if ($LASTEXITCODE -eq 0) {
        Write-Host "All tests passed." -ForegroundColor Green
        return $true
    } else {
        Write-Host "Some tests failed. Please check the output above." -ForegroundColor Red
        return $false
    }
}

# Function to build the project
function Build-Project {
    param (
        [string]$BuildType
    )
    Write-Host "Building Anya Core ($BuildType)..." -ForegroundColor Cyan
    if ($BuildType -eq "test") {
        cargo build
    } else {
        cargo build --release
    }
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Build successful." -ForegroundColor Green
        return $true
    } else {
        Write-Host "Build failed. Please check the output above." -ForegroundColor Red
        return $false
    }
}

# Function to sync Git repository
function Sync-GitRepository {
    Write-Host "Syncing Git repository..." -ForegroundColor Cyan

    # Ensure the correct remote URL is set
    $remoteUrl = "https://github.com/$($config.githubUser)/$($config.repoName).git"
    $currentRemote = git remote get-url origin
    if ($currentRemote -ne $remoteUrl) {
        git remote set-url origin $remoteUrl
        Write-Host "Updated remote URL to $remoteUrl" -ForegroundColor Yellow
    }

    # Fetch the latest changes
    git fetch origin
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to fetch from remote. Please check your internet connection and GitHub access." -ForegroundColor Red
        return $false
    }

    # Check if we need to pull changes
    $behindBy = git rev-list --count HEAD..origin/$(git rev-parse --abbrev-ref HEAD)
    if ($behindBy -gt 0) {
        Write-Host "Your branch is behind by $behindBy commits. Pulling changes..." -ForegroundColor Yellow
        git pull
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Failed to pull changes. Please resolve conflicts manually." -ForegroundColor Red
            return $false
        }
    }

    # Check if we need to push changes
    $aheadBy = git rev-list --count origin/$(git rev-parse --abbrev-ref HEAD)..HEAD
    if ($aheadBy -gt 0) {
        Write-Host "Your branch is ahead by $aheadBy commits. Pushing changes..." -ForegroundColor Yellow
        git push
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Failed to push changes. Please check your GitHub access and try again." -ForegroundColor Red
            return $false
        }
    }

    Write-Host "Repository is up to date." -ForegroundColor Green
    return $true
}

# Function to check system readiness
function Check-SystemReadiness {
    Write-Host "Checking system readiness..." -ForegroundColor Cyan

    # Check for outdated dependencies
    Write-Host "Checking for outdated dependencies..." -ForegroundColor Yellow
    cargo outdated
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Some dependencies are outdated. Consider updating them." -ForegroundColor Yellow
    }

    # Run cargo check
    Write-Host "Running cargo check..." -ForegroundColor Yellow
    cargo check
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Cargo check failed. Please fix the issues." -ForegroundColor Red
        return $false
    }

    # Run clippy
    Write-Host "Running clippy..." -ForegroundColor Yellow
    cargo clippy -- -D warnings
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Clippy found issues. Please fix them." -ForegroundColor Red
        return $false
    }

    # Run tests
    if (-not (Run-Tests)) {
        return $false
    }

    # Run security audit
    Write-Host "Running security audit..." -ForegroundColor Yellow
    cargo audit
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Security vulnerabilities found. Please address them." -ForegroundColor Red
        return $false
    }

    Write-Host "All readiness checks passed." -ForegroundColor Green
    return $true
}

# Function to analyze project and sync repositories
function Analyze-And-Sync {
    $allFiles = Get-AllFiles
    $cargoInfo = Get-CargoTomlInfo
    $rustInfo = Analyze-RustFiles
    $gitInfo = Get-GitStatus

    Write-Host "`nAnya Core Project Analysis Report" -ForegroundColor Green
    Write-Host "================================`n" -ForegroundColor Green

    Write-Host "Project Structure:" -ForegroundColor Yellow
    $allFiles | Group-Object Directory | ForEach-Object {
        Write-Host "  $($_.Name)"
        $_.Group | ForEach-Object {
            Write-Host "    $($_.Name)"
        }
    }

    Write-Host "`nCargo.toml Information:" -ForegroundColor Yellow
    if ($cargoInfo) {
        Write-Host "  Name: $($cargoInfo.Name)"
        Write-Host "  Version: $($cargoInfo.Version)"
        Write-Host "  Dependencies:"
        $cargoInfo.Dependencies -split "`n" | ForEach-Object {
            Write-Host "    $_"
        }
    } else {
        Write-Host "  Cargo.toml not found"
    }

    Write-Host "`nRust Code Analysis:" -ForegroundColor Yellow
    Write-Host "  Modules:"
    $rustInfo.Modules | ForEach-Object { Write-Host "    $_" }
    Write-Host "  Traits:"
    $rustInfo.Traits | ForEach-Object { Write-Host "    $_" }
    Write-Host "  Structs:"
    $rustInfo.Structs | ForEach-Object { Write-Host "    $_" }

    Write-Host "`nGit Information:" -ForegroundColor Yellow
    Write-Host "  Branch: $($gitInfo.Branch)"
    Write-Host "  Last Commit: $($gitInfo.LastCommit)"
    Write-Host "  Status:"
    if ($gitInfo.Status -eq "Clean") {
        Write-Host "    Working directory clean"
    } else {
        $gitInfo.Status -split "`n" | ForEach-Object {
            Write-Host "    $_"
        }
    }

    Write-Host "`nSyncing repositories..." -ForegroundColor Cyan
    Sync-GitRepository
}

# Function to update GitHub configuration
function Update-GitHubConfig {
    $config.githubUser = Read-Host "Enter your GitHub username"
    $config.githubToken = Read-Host "Enter your GitHub personal access token" -AsSecureString
    $BSTR = [System.Runtime.InteropServices.Marshal]::SecureStringToBSTR($config.githubToken)
    $config.githubToken = [System.Runtime.InteropServices.Marshal]::PtrToStringAuto($BSTR)
    $config.repoName = Read-Host "Enter the repository name (e.g., anya-core)"

    $config | ConvertTo-Json | Set-Content $configFile
    Write-Host "GitHub configuration updated and saved." -ForegroundColor Green
}

# Main menu function
function Show-Menu {
    Write-Host "`nAnya Core Project Manager" -ForegroundColor Cyan
    Write-Host "1. Analyze project and sync repositories"
    Write-Host "2. Set up environment"
    Write-Host "3. Check system readiness"
    Write-Host "4. Build test system"
    Write-Host "5. Build live system"
    Write-Host "6. Update GitHub configuration"
    Write-Host "7. Exit"
    $choice = Read-Host "`nEnter your choice"
    return $choice
}

# Main script
if (-not (Test-GitRepository)) {
    Write-Host "Initializing Git repository..." -ForegroundColor Yellow
    git init
    git remote add origin "https://github.com/$($config.githubUser)/$($config.repoName).git"
    Write-Host "Git repository initialized and remote added." -ForegroundColor Green
}

while ($true) {
    $choice = Show-Menu
    switch ($choice) {
        '1' { Analyze-And-Sync }
        '2' { Setup-Environment }
        '3' { Check-SystemReadiness }
        '4' { 
            if (Check-SystemReadiness) {
                Build-Project -BuildType "test"
            } else {
                Write-Host "System is not ready for build. Please address the issues above." -ForegroundColor Red
            }
        }
        '5' { 
            if (Check-SystemReadiness) {
                Build-Project -BuildType "live"
            } else {
                Write-Host "System is not ready for build. Please address the issues above." -ForegroundColor Red
            }
        }
        '6' { Update-GitHubConfig }
        '7' { 
            Write-Host "Exiting Anya Core Project Manager. Goodbye!" -ForegroundColor Cyan
            exit 
        }
        default { Write-Host "Invalid choice. Please try again." -ForegroundColor Red }
    }
    Write-Host "Press Enter to continue..."
    $null = Read-Host
}