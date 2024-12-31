# PowerShell script to set up GitHub Pages directory structure
$rootPath = "c:/Users/bmokoka/Downloads/OPSource/anya"
$docsPath = Join-Path $rootPath "docs"

# Create main directories
$directories = @(
    "docs",
    "docs/assets",
    "docs/assets/css",
    "docs/assets/js",
    "docs/assets/img",
    "docs/pages"
)

# Create directories
foreach ($dir in $directories) {
    $path = Join-Path $rootPath $dir
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force
        Write-Host "Created directory: $path"
    }
}

# Create empty files
$files = @(
    "docs/index.html",
    "docs/assets/css/styles.css",
    "docs/assets/js/main.js",
    "docs/pages/documentation.html",
    "docs/pages/features.html",
    "docs/pages/getting-started.html"
)

foreach ($file in $files) {
    $path = Join-Path $rootPath $file
    if (-not (Test-Path $path)) {
        New-Item -ItemType File -Path $path -Force
        Write-Host "Created file: $path"
    }
}

# Create .nojekyll file
$nojekyllPath = Join-Path $docsPath ".nojekyll"
if (-not (Test-Path $nojekyllPath)) {
    New-Item -ItemType File -Path $nojekyllPath -Force
    Write-Host "Created .nojekyll file"
}

Write-Host "`nGitHub Pages directory structure has been created successfully!"
Write-Host "Next steps:"
Write-Host "1. Add your HTML content to the created files"
Write-Host "2. Configure GitHub Pages in your repository settings"
Write-Host "3. Push the changes to your repository"
