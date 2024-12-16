# Set error action preference to stop on any error
$ErrorActionPreference = "Stop"

# Create a log file in Public directory
$logFile = "$env:PUBLIC\anya-install-progress.log"
"Starting installation at $(Get-Date)" | Out-File -FilePath $logFile -Force

function Write-Progress-Step {
    param(
        [string]$Message,
        [int]$StepNumber,
        [int]$TotalSteps = 8
    )
    $percentage = [math]::Round(($StepNumber / $TotalSteps) * 100)
    $status = "`n=== [$percentage%] $Message ==="
    $status | Out-File -FilePath $logFile -Append
    Write-Host $status -ForegroundColor Cyan
}

function Log-Output {
    param([string]$Message)
    $timestamp = Get-Date -Format "HH:mm:ss"
    $logMessage = "[$timestamp] $Message"
    $logMessage | Out-File -FilePath $logFile -Append
    Write-Host $logMessage
}

function Add-ToPath {
    param([string]$PathToAdd)
    
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    if ($currentPath -notlike "*$PathToAdd*") {
        [Environment]::SetEnvironmentVariable(
            "Path",
            "$currentPath;$PathToAdd",
            "Machine"
        )
        $status = "Added to PATH: $PathToAdd"
        Log-Output $status
        $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine")
    }
}

function Refresh-Environment {
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
}

Log-Output "Starting Development Environment Setup at $(Get-Date)"

# Step 1: Install Chocolatey
$currentStep = 1
Write-Progress-Step "Installing Package Manager (Chocolatey)" $currentStep
if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
    Log-Output "Installing Chocolatey..."
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
    $installScript = (New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1')
    iex $installScript | Out-String | Log-Output
    Refresh-Environment
    Add-ToPath "$env:ChocolateyInstall\bin"
} else {
    Log-Output "Chocolatey is already installed"
}

# Step 2: Install Visual Studio Build Tools
$currentStep++
Write-Progress-Step "Installing Visual Studio Build Tools" $currentStep
Log-Output "Installing Visual Studio Build Tools via Chocolatey..."
& "$env:ChocolateyInstall\bin\choco.exe" install visualstudio2022buildtools -y --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows10SDK" | Out-String | Log-Output

# Step 3: Install LLVM
$currentStep++
Write-Progress-Step "Installing LLVM" $currentStep
Log-Output "Installing LLVM via Chocolatey..."
& "$env:ChocolateyInstall\bin\choco.exe" install llvm -y | Out-String | Log-Output

# Step 4: Install/Update Rust
$currentStep++
Write-Progress-Step "Setting up Rust" $currentStep
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Log-Output "Installing Rust..."
    $rustupInit = "$env:TEMP\rustup-init.exe"
    (New-Object System.Net.WebClient).DownloadFile('https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe', $rustupInit)
    & $rustupInit -y --default-toolchain stable | Out-String | Log-Output
    Remove-Item $rustupInit
    Refresh-Environment
    Add-ToPath "$env:USERPROFILE\.cargo\bin"
} else {
    Log-Output "Rust is already installed"
}

# Step 5: Configure Rust
$currentStep++
Write-Progress-Step "Configuring Rust" $currentStep
Log-Output "Updating Rust..."
rustup update stable | Out-String | Log-Output
rustup default stable | Out-String | Log-Output
rustup target add x86_64-pc-windows-msvc | Out-String | Log-Output

# Step 6: Configure Environment PATH
$currentStep++
Write-Progress-Step "Configuring Environment PATH" $currentStep
$vsPath = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC"
if (Test-Path $vsPath) {
    $latestVersion = (Get-ChildItem $vsPath | Sort-Object Name -Descending)[0].Name
    Add-ToPath "$vsPath\$latestVersion\bin\Hostx64\x64"
}
Add-ToPath "C:\Program Files\LLVM\bin"

# Step 7: Configure Cargo
$currentStep++
Write-Progress-Step "Configuring Cargo" $currentStep
$cargoConfig = @"
[target.x86_64-pc-windows-msvc]
linker = "lld-link"
rustflags = [
    "-C", "link-arg=/NODEFAULTLIB:libcmt.lib",
    "-C", "link-arg=/NODEFAULTLIB:libvcruntime.lib",
    "-C", "link-arg=/NODEFAULTLIB:libucrt.lib",
    "-C", "link-arg=/DEFAULTLIB:msvcrt.lib"
]
"@

$cargoConfigPath = "$env:USERPROFILE\.cargo\config"
if (-not (Test-Path $cargoConfigPath)) {
    New-Item -Path "$env:USERPROFILE\.cargo" -ItemType Directory -Force | Out-Null
}
$cargoConfig | Out-File -FilePath $cargoConfigPath -Encoding utf8 -Force
Log-Output "Created Cargo config at $cargoConfigPath"

# Step 8: Verify Installation
$currentStep++
Write-Progress-Step "Verifying Installation" $currentStep
Log-Output "Verifying installed components:"
Log-Output "Rust Version:"
rustc --version | Out-String | Log-Output
Log-Output "Cargo Version:"
cargo --version | Out-String | Log-Output
Log-Output "LLVM Version:"
clang --version | Out-String | Log-Output

Log-Output "Installation completed at $(Get-Date)"
Log-Output "Please restart your terminal to ensure all environment variables are updated."
