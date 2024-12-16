# auto-organize.ps1

# Configuration
$CONFIG = @{
    BackupDir = "backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
    LogFile = "reorganization.log"
}

# Initialize logging
function Write-Log {
    param($Message)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    "$timestamp - $Message" | Tee-Object -FilePath $CONFIG.LogFile -Append
}

# Function to merge file content
function Merge-FileContent {
    param(
        $SourcePath,
        $DestPath
    )
    
    if (!(Test-Path $SourcePath)) {
        Write-Log "Source file not found: $SourcePath"
        return
    }

    if (Test-Path $DestPath) {
        # File exists, merge content
        $sourceContent = Get-Content $SourcePath -Raw
        $destContent = Get-Content $DestPath -Raw
        
        # Simple merge strategy - append non-duplicate content
        $mergedContent = @()
        $sourceLines = $sourceContent -split "`n"
        $destLines = $destContent -split "`n"
        
        foreach ($line in $sourceLines) {
            if ($destLines -notcontains $line.Trim()) {
                $mergedContent += $line
            }
        }
        
        if ($mergedContent.Count -gt 0) {
            $mergedContent | Add-Content $DestPath
            Write-Log "Merged new content into $DestPath"
        }
    } else {
        # File doesn't exist, just copy
        Copy-Item -Path $SourcePath -Destination $DestPath -Force
        Write-Log "Created new file: $DestPath"
    }
}

# Function to fix code alignment
function Fix-CodeAlignment {
    param($FilePath)
    
    if (!(Test-Path $FilePath)) { return }
    
    $content = Get-Content $FilePath -Raw
    
    # Fix indentation
    $content = $content -replace "`t", "    "
    
    # Fix empty lines
    $content = $content -replace "`r`n`r`n`r`n+", "`r`n`r`n"
    
    # Fix trailing whitespace
    $content = $content -replace " +$", ""
    
    # Fix imports ordering
    $lines = $content -split "`r`n"
    $useStatements = $lines | Where-Object { $_ -match "^use " } | Sort-Object
    $otherLines = $lines | Where-Object { $_ -notmatch "^use " }
    $content = ($useStatements + "" + $otherLines) -join "`r`n"
    
    Set-Content -Path $FilePath -Value $content -NoNewline
    Write-Log "Fixed code alignment in $FilePath"
}

# Main execution
Write-Log "Starting auto-organization"

# Create backup
Copy-Item -Path "." -Destination $CONFIG.BackupDir -Recurse
Write-Log "Created backup in $CONFIG.BackupDir"

# Process each file
Get-ChildItem -Recurse -File -Filter "*.rs" | ForEach-Object {
    $sourcePath = $_.FullName
    $relativePath = $_.FullName.Replace($PWD.Path, "").TrimStart("\")
    $destPath = Join-Path "anya-core/src" $relativePath
    
    # Create destination directory if it doesn't exist
    $destDir = Split-Path -Parent $destPath
    if (!(Test-Path $destDir)) {
        New-Item -ItemType Directory -Path $destDir -Force | Out-Null
    }
    
    # Merge content and fix alignment
    Merge-FileContent -SourcePath $sourcePath -DestPath $destPath
    Fix-CodeAlignment -FilePath $destPath
}

Write-Log "Auto-organization complete"