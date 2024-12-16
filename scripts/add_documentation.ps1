#!/usr/bin/env pwsh

# Script to add documentation to Rust files
$ErrorActionPreference = "Stop"

Write-Host "Adding documentation to Rust files..."

# Get template content
$templatePath = "docs/templates/module_template.md"
$template = Get-Content $templatePath -Raw

# Get all Rust files
$rustFiles = Get-ChildItem -Path "src" -Recurse -Filter "*.rs"

foreach ($file in $rustFiles) {
    Write-Host "Processing $($file.FullName)..."
    
    # Read file content
    $content = Get-Content $file.FullName -Raw
    
    # Skip if file already has documentation
    if ($content -match '//! Module documentation') {
        Write-Host "Documentation already exists, skipping..."
        continue
    }
    
    # Generate documentation from template
    $moduleName = $file.BaseName
    $modulePath = $file.FullName.Replace((Get-Location), "").TrimStart("\")
    
    $documentation = @"
//! Module documentation for `$moduleName`
//!
//! # Overview
//! This module is part of the Anya Core project, located at `$modulePath`.
//!
//! # Architecture
//! [Add module-specific architecture details]
//!
//! # API Reference
//! [Document public functions and types]
//!
//! # Usage Examples
//! ```rust
//! // Add usage examples
//! ```
//!
//! # Error Handling
//! This module uses proper error handling with Result types.
//!
//! # Security Considerations
//! [Document security features and considerations]
//!
//! # Performance
//! [Document performance characteristics]

"@
    
    # Add documentation to file
    $content = "$documentation`n$content"
    $content | Set-Content $file.FullName
}

Write-Host "Done adding documentation."
