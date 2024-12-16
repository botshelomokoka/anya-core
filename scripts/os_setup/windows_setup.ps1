# Windows-specific setup for Anya Project
param(
    [string]$ProjectRoot = (Resolve-Path (Join-Path $PSScriptRoot "../..")).Path
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

# Import common utilities
. (Join-Path $PSScriptRoot "../lib/common.ps1")

function Initialize-WindowsEnvironment {
    Write-Log "Initializing Windows environment..." -Level Info
    
    # Set execution policy for PowerShell scripts
    if ((Get-ExecutionPolicy) -ne "RemoteSigned") {
        Write-Log "Setting PowerShell execution policy to RemoteSigned..." -Level Info
        Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force
    }
    
    # Set up PATH environment
    $paths = @(
        "$env:USERPROFILE\.cargo\bin",
        "C:\Program Files\OpenSSL-Win64\bin",
        "C:\Program Files\nodejs"
    )
    
    foreach ($path in $paths) {
        if ($env:Path -notlike "*$path*") {
            Write-Log "Adding $path to PATH..." -Level Info
            $env:Path = "$path;$env:Path"
            [Environment]::SetEnvironmentVariable(
                "Path",
                "$path;$([Environment]::GetEnvironmentVariable('Path', 'User'))",
                "User"
            )
        }
    }
}

function Set-WindowsDefender {
    Write-Log "Configuring Windows Defender..." -Level Info
    
    # Add project directory to exclusions to improve build performance
    try {
        Add-MpPreference -ExclusionPath $ProjectRoot -ErrorAction SilentlyContinue
        Write-Log "Added project directory to Windows Defender exclusions" -Level Info
    } catch {
        Write-Log "Failed to configure Windows Defender: $_" -Level Warning
    }
}

function Set-WindowsPerformance {
    Write-Log "Optimizing Windows performance settings..." -Level Info
    
    # Set power plan to high performance
    try {
        powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c
        Write-Log "Set power plan to high performance" -Level Info
    } catch {
        Write-Log "Failed to set power plan: $_" -Level Warning
    }
    
    # Configure virtual memory
    try {
        $computerSystem = Get-WmiObject -Class Win32_ComputerSystem
        $totalRam = [Math]::Round($computerSystem.TotalPhysicalMemory / 1GB)
        $pagefile = Get-WmiObject -Class Win32_PageFileSetting
        if ($pagefile) {
            $pagefile.InitialSize = $totalRam * 1024
            $pagefile.MaximumSize = $totalRam * 2048
            $pagefile.Put()
        }
        Write-Log "Configured virtual memory settings" -Level Info
    } catch {
        Write-Log "Failed to configure virtual memory: $_" -Level Warning
    }
}

function Install-WindowsTools {
    Write-Log "Installing Windows-specific tools..." -Level Info
    
    # Install Windows Terminal if not present
    if (-not (Get-AppxPackage -Name Microsoft.WindowsTerminal)) {
        try {
            winget install --id Microsoft.WindowsTerminal -e
            Write-Log "Installed Windows Terminal" -Level Info
        } catch {
            Write-Log "Failed to install Windows Terminal: $_" -Level Warning
        }
    }
    
    # Install PowerShell 7 if not present
    if (-not (Get-Command pwsh -ErrorAction SilentlyContinue)) {
        try {
            winget install --id Microsoft.PowerShell -e
            Write-Log "Installed PowerShell 7" -Level Info
        } catch {
            Write-Log "Failed to install PowerShell 7: $_" -Level Warning
        }
    }
}

function Set-GitConfig {
    Write-Log "Configuring Git for Windows..." -Level Info
    
    # Configure Git to handle line endings
    git config --global core.autocrlf true
    
    # Enable long paths
    git config --global core.longpaths true
    
    # Configure Git Credential Manager
    git config --global credential.helper manager-core
    
    Write-Log "Git configuration completed" -Level Info
}

function Main {
    Write-Log "Starting Windows-specific setup..." -Level Info
    
    # Check if running as administrator
    $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    if (-not $isAdmin) {
        Write-Log "Some operations may require administrator privileges" -Level Warning
    }
    
    Initialize-WindowsEnvironment
    Set-WindowsDefender
    Set-WindowsPerformance
    Install-WindowsTools
    Set-GitConfig
    
    Write-Log "Windows-specific setup completed successfully" -Level Info
}

# Run main if script is executed directly
if ($MyInvocation.InvocationName -eq $MyInvocation.MyCommand.Path) {
    Main
}
