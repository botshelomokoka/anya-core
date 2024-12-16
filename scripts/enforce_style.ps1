#!/usr/bin/env pwsh

# Script to enforce consistent code style across the Anya codebase
$ErrorActionPreference = "Stop"

# Install required tools if not present
function Install-RequiredTools {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error "Rust toolchain not found. Please install Rust first."
        exit 1
    }
    
    cargo install rustfmt
    cargo install clippy
}

# Format all Rust files
function Format-RustCode {
    Write-Host "Formatting Rust code..."
    cargo fmt --all
}

# Run clippy with strict settings
function Run-Clippy {
    Write-Host "Running Clippy..."
    cargo clippy -- -D warnings
}

# Check documentation coverage
function Check-Documentation {
    Write-Host "Checking documentation..."
    cargo doc --no-deps --all-features
    
    # Find files missing documentation
    Get-ChildItem -Path "src" -Recurse -Filter "*.rs" | ForEach-Object {
        $docCount = Select-String -Path $_.FullName -Pattern "///" | Measure-Object | Select-Object -ExpandProperty Count
        if ($docCount -eq 0) {
            Write-Warning "No documentation found in $($_.FullName)"
        }
    }
}

# Enforce consistent error handling
function Check-ErrorHandling {
    Write-Host "Checking error handling patterns..."
    $errorPatterns = @(
        'unwrap\(',
        'expect\(',
        'panic!\('
    )
    
    Get-ChildItem -Path "src" -Recurse -Filter "*.rs" | ForEach-Object {
        foreach ($pattern in $errorPatterns) {
            $matches = Select-String -Path $_.FullName -Pattern $pattern
            if ($matches) {
                Write-Warning "Found unsafe error handling in $($_.FullName):"
                $matches | ForEach-Object { Write-Warning "  Line $($_.LineNumber): $($_.Line.Trim())" }
            }
        }
    }
}

# Check async/await consistency
function Check-AsyncPatterns {
    Write-Host "Checking async/await patterns..."
    Get-ChildItem -Path "src" -Recurse -Filter "*.rs" | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        if ($content -match "async" -and -not ($content -match "\.await")) {
            Write-Warning "Possible missing .await in $($_.FullName)"
        }
    }
}

# Main execution
try {
    Install-RequiredTools
    Format-RustCode
    Run-Clippy
    Check-Documentation
    Check-ErrorHandling
    Check-AsyncPatterns
    
    Write-Host "Style enforcement completed successfully!"
} catch {
    Write-Error "Error during style enforcement: $_"
    exit 1
}
