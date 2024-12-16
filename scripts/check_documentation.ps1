#!/usr/bin/env pwsh

# Script to check documentation coverage and quality
$ErrorActionPreference = "Stop"

Write-Host "Checking documentation coverage..."

$exitCode = 0
$rustFiles = Get-ChildItem -Path "src" -Recurse -Filter "*.rs"

foreach ($file in $rustFiles) {
    Write-Host "Checking $($file.FullName)..."
    
    $content = Get-Content $file.FullName -Raw
    
    # Check for module documentation
    if (-not ($content -match '//! Module documentation')) {
        Write-Host "ERROR: Missing module documentation in $($file.FullName)" -ForegroundColor Red
        $exitCode = 1
        continue
    }
    
    # Check for function documentation
    $functions = Select-String -Path $file.FullName -Pattern '^(\s*)pub(\s+)fn' -AllMatches
    foreach ($function in $functions.Matches) {
        $content = Get-Content $file.FullName
        $lineNumber = 1
        foreach ($line in $content) {
            if ($line -match [regex]::Escape($function.Value)) {
                if ($lineNumber -gt 2) {
                    $previousLine = $content[$lineNumber - 2]
                    if (-not ($previousLine -match '///')) {
                        Write-Host "ERROR: Missing function documentation at line $lineNumber in $($file.FullName)" -ForegroundColor Red
                        $exitCode = 1
                    }
                }
                break
            }
            $lineNumber++
        }
    }
    
    # Check for type documentation
    $types = Select-String -Path $file.FullName -Pattern '^(\s*)pub(\s+)(struct|enum|type)' -AllMatches
    foreach ($type in $types.Matches) {
        $content = Get-Content $file.FullName
        $lineNumber = 1
        foreach ($line in $content) {
            if ($line -match [regex]::Escape($type.Value)) {
                if ($lineNumber -gt 2) {
                    $previousLine = $content[$lineNumber - 2]
                    if (-not ($previousLine -match '///')) {
                        Write-Host "ERROR: Missing type documentation at line $lineNumber in $($file.FullName)" -ForegroundColor Red
                        $exitCode = 1
                    }
                }
                break
            }
            $lineNumber++
        }
    }
}

Write-Host "Documentation check complete."
exit $exitCode
