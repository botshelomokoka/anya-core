# PowerShell script to update symbolic links and system map
param (
    [string]$rootDir = (Get-Location)
)

# Function to create symbolic links
function Create-SymLink {
    param (
        [string]$source,
        [string]$target
    )
    
    if (Test-Path $target) {
        Write-Host "Removing existing link: $target"
        Remove-Item $target -Force -Recurse
    }
    
    Write-Host "Creating symbolic link: $source -> $target"
    New-Item -ItemType Junction -Path $target -Target $source -Force
}

# Function to update last modified date in markdown files
function Update-MarkdownDate {
    param (
        [string]$markdownFile
    )
    
    if (Test-Path $markdownFile) {
        $content = Get-Content $markdownFile -Raw
        $newDate = Get-Date -Format "yyyy-MM-dd"
        $content = $content -replace "Last updated: \d{4}-\d{2}-\d{2}", "Last updated: $newDate"
        Set-Content -Path $markdownFile -Value $content
        Write-Host "Updated timestamp in: $markdownFile"
    }
}

# Function to verify index links
function Verify-IndexLinks {
    param (
        [string]$indexFile
    )
    
    if (Test-Path $indexFile) {
        $content = Get-Content $indexFile -Raw
        $pattern = '\[([^\]]+)\]\(([^)]+)\)'
        $matches = [regex]::Matches($content, $pattern)
        $brokenLinks = @()
        
        foreach ($match in $matches) {
            $linkText = $match.Groups[1].Value
            $linkPath = $match.Groups[2].Value
            
            if (-not $linkPath.StartsWith("http")) {
                $fullPath = Join-Path (Split-Path $indexFile) $linkPath.TrimStart("./")
                if (-not (Test-Path $fullPath)) {
                    $brokenLinks += "$linkText -> $linkPath"
                }
            }
        }
        
        if ($brokenLinks.Count -gt 0) {
            Write-Warning "Broken links found in $(Split-Path $indexFile -Leaf)"
            $brokenLinks | ForEach-Object { Write-Warning "  $_" }
        }
    }
}

# Main execution
Write-Host "Updating Anya system links and documentation..."

# Ensure anya directory exists
$anyaDir = Join-Path -Path $rootDir -ChildPath "anya"
if (-not (Test-Path $anyaDir)) {
    New-Item -ItemType Directory -Path $anyaDir -Force
}

# Create symbolic links for main components
$components = @{
    "dash33" = "dash33"
    "enterprise" = "enterprise"
    "mobile" = "mobile"
}

foreach ($comp in $components.GetEnumerator()) {
    $source = Join-Path -Path $rootDir -ChildPath $comp.Key
    $target = Join-Path -Path $anyaDir -ChildPath $comp.Value
    
    if (Test-Path $source) {
        Create-SymLink -source $source -target $target
    } else {
        Write-Warning "Source directory not found: $source"
    }
}

# Update timestamps in index and system map files
$indexFiles = @(
    (Join-Path $rootDir "INDEX.md"),
    (Join-Path $anyaDir "INDEX.md"),
    (Join-Path $rootDir "dash33/INDEX.md"),
    (Join-Path $rootDir "enterprise/INDEX.md"),
    (Join-Path $rootDir "mobile/INDEX.md")
)

$systemMapPath = Join-Path -Path $anyaDir -ChildPath "SYSTEM_MAP.md"
$indexFiles += $systemMapPath

foreach ($file in $indexFiles) {
    Update-MarkdownDate -markdownFile $file
    Verify-IndexLinks -indexFile $file
}

Write-Host "System links and documentation updated successfully"
