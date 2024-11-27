#!/usr/bin/env pwsh

# Script to check for unsafe error handling patterns
$ErrorActionPreference = "Stop"

Write-Host "Checking for unsafe error handling patterns..."

$exitCode = 0
$rustFiles = Get-ChildItem -Path "src" -Recurse -Filter "*.rs"

$unsafePatterns = @(
    '\bunwrap\(\)',
    '\bexpect\(',
    '\bpanic!\(',
    '\bunreachable!\(',
    '\bunwrap_or\(',
    '\bunwrap_or_else\('
)

foreach ($file in $rustFiles) {
    Write-Host "Checking $($file.FullName)..."
    
    $content = Get-Content $file.FullName
    
    $lineNumber = 1
    foreach ($line in $content) {
        foreach ($pattern in $unsafePatterns) {
            if ($line -match $pattern) {
                # Skip if in test module or with specific allow attribute
                if (-not ($line -match '#\[test\]' -or $line -match '#\[allow\(clippy::unwrap_used\)\]')) {
                    Write-Host "ERROR: Unsafe error handling pattern '$pattern' found at line $lineNumber in $($file.FullName)" -ForegroundColor Red
                    $exitCode = 1
                }
            }
        }
        $lineNumber++
    }
}

Write-Host "Error handling check complete."
exit $exitCode
