#!/usr/bin/env pwsh

# Script to fix unsafe error handling patterns
$ErrorActionPreference = "Stop"

Write-Host "Scanning for unsafe error handling patterns..."

# Get all Rust files
$rustFiles = Get-ChildItem -Path "src" -Recurse -Filter "*.rs"

foreach ($file in $rustFiles) {
    Write-Host "Processing $($file.FullName)..."
    
    # Read file content
    $content = Get-Content $file.FullName -Raw
    
    # Replace unsafe patterns with safe alternatives
    $content = $content -replace '\.unwrap\(\)', '?'
    $content = $content -replace '\.expect\([^)]+\)', '?'
    
    # Add error propagation
    if ($content -notmatch 'use std::error::Error;') {
        $content = "use std::error::Error;`n$content"
    }
    
    # Add Result return type if not present
    if ($content -match 'fn [^{]+{' -and $content -notmatch '-> Result<') {
        $content = $content -replace 'fn ([^{]+){', 'fn $1 -> Result<(), Box<dyn Error>> {'
    }
    
    # Save changes
    $content | Set-Content $file.FullName
}

Write-Host "Done fixing error handling patterns."
