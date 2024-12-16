# PowerShell script for code organization and realignment

# Configuration
$CONFIG = @{
    BackupDir = "backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
    LogFile = "code_reorganization.log"
}

# Initialize logging
function Write-Log {
    param($Message)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    "$timestamp - $Message" | Tee-Object -FilePath $CONFIG.LogFile -Append
}

# File mappings with validation rules
$FILE_MAPPINGS = @{
    # Bitcoin module
    "src/bitcoin/mod.rs" = @{
        Destination = "anya-core/src/bitcoin/mod.rs"
        Required = $true
        ValidateContent = { param($content) $content -match "pub mod core;" }
    }
    "src/bitcoin/core/mod.rs" = @{
        Destination = "anya-core/src/bitcoin/core/mod.rs"
        Required = $true
        ValidateContent = { param($content) $content -match "use bitcoin::" }
    }
    # Add more mappings...
}

# Create backup
function Create-Backup {
    param($SourcePath)
    
    Write-Log "Creating backup of $SourcePath"
    if (!(Test-Path $CONFIG.BackupDir)) {
        New-Item -ItemType Directory -Path $CONFIG.BackupDir | Out-Null
    }
    
    try {
        Copy-Item -Path $SourcePath -Destination $CONFIG.BackupDir -Recurse -Force
        Write-Log "Backup created successfully"
        return $true
    }
    catch {
        Write-Log "ERROR: Failed to create backup: $_"
        return $false
    }
}

# Validate file content
function Test-FileContent {
    param($FilePath, $ValidationRule)
    
    if (!(Test-Path $FilePath)) { return $false }
    
    try {
        $content = Get-Content $FilePath -Raw
        & $ValidationRule $content
    }
    catch {
        Write-Log "ERROR: Content validation failed for $FilePath : $_"
        return $false
    }
}

# Realign code
function Update-CodeAlignment {
    param($FilePath)
    
    try {
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
        Write-Log "Code realigned in $FilePath"
        return $true
    }
    catch {
        Write-Log "ERROR: Failed to realign code in $FilePath : $_"
        return $false
    }
}

# Main execution
Write-Log "Starting code reorganization"

# Create backup first
if (!(Create-Backup "src")) {
    Write-Log "ERROR: Backup failed, aborting"
    exit 1
}

# Process each mapping
foreach ($mapping in $FILE_MAPPINGS.GetEnumerator()) {
    $source = $mapping.Key
    $dest = $mapping.Value.Destination
    $required = $mapping.Value.Required
    $validator = $mapping.Value.ValidateContent
    
    Write-Log "Processing $source -> $dest"
    
    # Check if destination exists
    if (Test-Path $dest) {
        Write-Log "Destination exists, checking alignment"
        if (!(Test-FileContent $dest $validator)) {
            Write-Log "Content validation failed, realigning"
            Update-CodeAlignment $dest
        }
        continue
    }
    
    # Create destination directory if needed
    $destDir = Split-Path -Parent $dest
    if (!(Test-Path $destDir)) {
        New-Item -ItemType Directory -Path $destDir -Force | Out-Null
    }
    
    # Move file if source exists
    if (Test-Path $source) {
        Copy-Item -Path $source -Destination $dest -Force
        Update-CodeAlignment $dest
    }
    elseif ($required) {
        Write-Log "ERROR: Required file $source not found"
        exit 1
    }
}

Write-Log "Code reorganization completed"