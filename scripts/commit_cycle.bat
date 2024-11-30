@echo off
setlocal enabledelayedexpansion

:: Commit cycle script for Anya project
:: Usage: commit_cycle.bat <commit_message>

if "%~1"=="" (
    echo Error: Please provide a commit message
    echo Usage: commit_cycle.bat "<commit_message>"
    exit /b 1
)

set "COMMIT_MSG=%~1"
set "ROOT_DIR=%CD%"
set "SUBMODULES=dash33 dependencies enterprise"

echo 🔄 Starting commit cycle...

:: Function to commit changes in a repository
:commit_repo
set "REPO_PATH=%~1"
set "REPO_NAME=%~2"
set "MESSAGE=%~3"

echo Processing %REPO_NAME%...
cd "%REPO_PATH%" || exit /b 1

:: Pull latest changes
git pull origin main
if errorlevel 1 (
    echo Failed to pull %REPO_NAME%
    exit /b 1
)

:: Add and commit changes
git add .
if errorlevel 1 (
    echo Failed to stage changes in %REPO_NAME%
    exit /b 1
)

:: Check if there are changes to commit
git diff --cached --quiet
if errorlevel 1 (
    :: Changes exist, commit them
    git commit -m "%MESSAGE%"
    if errorlevel 1 (
        echo Failed to commit %REPO_NAME%
        exit /b 1
    )
    
    git push origin main
    if errorlevel 1 (
        echo Failed to push %REPO_NAME%
        exit /b 1
    )
    echo ✅ %REPO_NAME%: Changes committed and pushed
) else (
    echo ℹ️ %REPO_NAME%: No changes to commit
)
exit /b 0

:: 1. Process submodules first
for %%s in (%SUBMODULES%) do (
    call :commit_repo "%ROOT_DIR%\%%s" "%%s" "feat(%%s): %COMMIT_MSG%"
    if errorlevel 1 exit /b 1
)

:: 2. Update main repository submodule references
cd "%ROOT_DIR%"
git add %SUBMODULES%
if errorlevel 1 (
    echo Failed to update submodule references
    exit /b 1
)
git commit -m "chore: Update submodule references" 2>nul

:: 3. Commit main repository changes
call :commit_repo "%ROOT_DIR%" "main" "feat: %COMMIT_MSG%"
if errorlevel 1 exit /b 1

echo ✅ Commit cycle completed successfully
exit /b 0
