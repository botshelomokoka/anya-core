#!/usr/bin/env pwsh

# Script to align and reorganize the Anya codebase
# Run with elevated privileges

$ErrorActionPreference = "Stop"

# Paths
$SRC_DIR = Join-Path $PSScriptRoot ".." "src"
$DOCS_DIR = Join-Path $PSScriptRoot ".." "docs"

# Create new directory structure
$NEW_DIRS = @(
    "core",           # Core Bitcoin and blockchain functionality
    "protocols",      # Protocol implementations (Lightning, RGB, DLC)
    "security",       # Unified security features
    "ml",            # Machine learning and AI
    "infrastructure" # Shared infrastructure code
)

foreach ($dir in $NEW_DIRS) {
    $path = Join-Path $SRC_DIR $dir
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path
    }
}

# Move and reorganize files
function Move-ModuleFiles {
    param (
        [string]$SourcePattern,
        [string]$TargetDir
    )
    
    Get-ChildItem -Path $SRC_DIR -Recurse -File -Filter $SourcePattern | 
    ForEach-Object {
        $targetPath = Join-Path $TargetDir $_.Name
        if (-not (Test-Path $targetPath)) {
            Move-Item $_.FullName $targetPath -Force
        }
    }
}

# Reorganize core functionality
Move-ModuleFiles -SourcePattern "bitcoin*.rs" -TargetDir (Join-Path $SRC_DIR "core")
Move-ModuleFiles -SourcePattern "blockchain*.rs" -TargetDir (Join-Path $SRC_DIR "core")

# Reorganize protocols
Move-ModuleFiles -SourcePattern "lightning*.rs" -TargetDir (Join-Path $SRC_DIR "protocols")
Move-ModuleFiles -SourcePattern "rgb*.rs" -TargetDir (Join-Path $SRC_DIR "protocols")
Move-ModuleFiles -SourcePattern "dlc*.rs" -TargetDir (Join-Path $SRC_DIR "protocols")

# Reorganize security
Move-ModuleFiles -SourcePattern "*security*.rs" -TargetDir (Join-Path $SRC_DIR "security")
Move-ModuleFiles -SourcePattern "*auth*.rs" -TargetDir (Join-Path $SRC_DIR "security")

# Reorganize ML
Move-ModuleFiles -SourcePattern "ml_*.rs" -TargetDir (Join-Path $SRC_DIR "ml")
Move-ModuleFiles -SourcePattern "ai_*.rs" -TargetDir (Join-Path $SRC_DIR "ml")

# Generate module documentation
function Add-ModuleDoc {
    param (
        [string]$ModulePath,
        [string]$Description
    )
    
    $docPath = Join-Path $DOCS_DIR "modules" "$($ModulePath).md"
    if (-not (Test-Path $docPath)) {
        $content = @"
# $ModulePath Module

$Description

## Overview

This module is part of the Anya Core system and provides...

## Components

- List key components here

## Usage

Example usage and API documentation...

## Architecture

Describe the module's architecture and key design decisions...
"@
        Set-Content -Path $docPath -Value $content
    }
}

# Add documentation for each module
Add-ModuleDoc -ModulePath "core" -Description "Core Bitcoin and blockchain functionality"
Add-ModuleDoc -ModulePath "protocols" -Description "Protocol implementations including Lightning, RGB, and DLC"
Add-ModuleDoc -ModulePath "security" -Description "Security features and authentication"
Add-ModuleDoc -ModulePath "ml" -Description "Machine learning and AI capabilities"
Add-ModuleDoc -ModulePath "infrastructure" -Description "Shared infrastructure components"

Write-Host "Code alignment completed. Please review changes and update module imports as needed."
