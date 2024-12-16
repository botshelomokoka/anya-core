# PowerShell script to verify documentation links
param (
    [string]$rootDir = (Get-Location)
)

# Function to extract markdown links
function Get-MarkdownLinks {
    param (
        [string]$content
    )
    
    $pattern = '\[([^\]]+)\]\(([^)]+)\)'
    $matches = [regex]::Matches($content, $pattern)
    return $matches
}

# Function to verify file exists
function Test-DocumentLink {
    param (
        [string]$link,
        [string]$baseDir
    )
    
    if ($link.StartsWith("http")) {
        return $true  # Skip external links
    }
    
    $fullPath = Join-Path $baseDir $link.TrimStart("./")
    return Test-Path $fullPath
}

# Main execution
Write-Host "Verifying documentation links..."

$systemMapPath = Join-Path $rootDir "anya" "SYSTEM_MAP.md"
$brokenLinks = @()

if (Test-Path $systemMapPath) {
    $content = Get-Content $systemMapPath -Raw
    $links = Get-MarkdownLinks $content
    
    foreach ($link in $links) {
        $linkText = $link.Groups[1].Value
        $linkPath = $link.Groups[2].Value
        
        if (-not (Test-DocumentLink -link $linkPath -baseDir (Split-Path $systemMapPath))) {
            $brokenLinks += @{
                Text = $linkText
                Path = $linkPath
            }
        }
    }
}

if ($brokenLinks.Count -gt 0) {
    Write-Host "Found broken links:"
    foreach ($link in $brokenLinks) {
        Write-Host "- [$($link.Text)]($($link.Path))"
    }
    exit 1
} else {
    Write-Host "All documentation links are valid"
    exit 0
}
