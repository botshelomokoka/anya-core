#!/bin/bash
# Bitcoin Core build and script standards for Anya project

# Import common utilities
# shellcheck source=./common.sh
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Bitcoin Core style constants
readonly BTC_SCRIPT_VERSION="1.0.0"
readonly BTC_MIN_BASH_VERSION="4.0.0"
readonly BTC_REQUIRED_TOOLS=("autoconf" "automake" "berkeley-db" "boost" "libevent" "qt5")

# Check bash version
check_bash_version() {
    if [[ "${BASH_VERSINFO[0]}.${BASH_VERSINFO[1]}.${BASH_VERSINFO[2]}" < "${BTC_MIN_BASH_VERSION}" ]]; then
        log ERROR "Bash version ${BTC_MIN_BASH_VERSION} or higher required"
        return "${EXIT_NOT_SUPPORTED}"
    fi
}

# Verify build environment
verify_build_env() {
    check_dependencies "${BTC_REQUIRED_TOOLS[@]}" || return $?
    
    # Check for required environment variables
    local required_vars=("BITCOIN_ROOT" "BITCOIN_CONFIG")
    for var in "${required_vars[@]}"; do
        if [[ -z "${!var}" ]]; then
            log ERROR "Required environment variable $var is not set"
            return "${EXIT_BAD_CONFIG}"
        fi
    done
}

# Script style enforcement
enforce_script_style() {
    local script_file=$1
    
    # Check shebang
    if ! grep -q '^#!/bin/bash$' "$script_file"; then
        log ERROR "Missing or incorrect shebang in $script_file"
        return "${EXIT_BAD_CONFIG}"
    fi
    
    # Check for 'set -e'
    if ! grep -q '^set -e$' "$script_file"; then
        log ERROR "Missing 'set -e' in $script_file"
        return "${EXIT_BAD_CONFIG}"
    fi
    
    # Check for shellcheck directives
    if ! grep -q '^# shellcheck' "$script_file"; then
        log WARN "Missing shellcheck directives in $script_file"
    fi
}

# Function naming convention
btc_function_name() {
    local prefix=$1
    local name=$2
    echo "${prefix}_${name}"
}

# Error handling
btc_error_handler() {
    local exit_code=$?
    local line_no=$1
    log ERROR "Error on line ${line_no}: Exit code ${exit_code}"
    return "${exit_code}"
}

# Set error trap
trap 'btc_error_handler ${LINENO}' ERR

# Version control standards
btc_check_git_config() {
    local repo_path=${1:-.}
    
    # Check Git configuration
    local git_configs=(
        "core.autocrlf=false"
        "core.eol=lf"
        "core.fileMode=false"
        "core.symlinks=false"
    )
    
    for config in "${git_configs[@]}"; do
        local key="${config%=*}"
        local value="${config#*=}"
        local current
        current=$(git -C "$repo_path" config --get "$key")
        
        if [[ "$current" != "$value" ]]; then
            log WARN "Git config $key should be $value (current: $current)"
            git -C "$repo_path" config "$key" "$value"
        fi
    done
}

# Build system integration
btc_configure_build() {
    local build_dir=$1
    local options=("${@:2}")
    
    # Standard build options
    local default_options=(
        "--with-gui=qt5"
        "--with-boost"
        "--with-libevent"
        "--with-berkeley-db"
    )
    
    # Combine default and custom options
    local all_options=("${default_options[@]}" "${options[@]}")
    
    # Run configure
    (
        cd "$build_dir" || exit "${EXIT_FAILURE}"
        ./configure "${all_options[@]}"
    )
}

# Test framework integration
btc_run_tests() {
    local test_dir=$1
    local test_filter=${2:-*}
    
    # Run tests with coverage
    if command -v lcov &> /dev/null; then
        lcov --capture --directory "$test_dir" --output-file coverage.info
        genhtml coverage.info --output-directory coverage
    fi
    
    # Run selected tests
    if [[ -f "$test_dir/test_runner.py" ]]; then
        python3 "$test_dir/test_runner.py" "$test_filter"
    fi
}

# Documentation standards
btc_check_docs() {
    local docs_dir=$1
    
    # Check for required documentation files
    local required_docs=(
        "README.md"
        "CONTRIBUTING.md"
        "SECURITY.md"
        "LICENSE"
    )
    
    for doc in "${required_docs[@]}"; do
        if [[ ! -f "$docs_dir/$doc" ]]; then
            log WARN "Missing documentation file: $doc"
        fi
    done
}

# Export functions
export -f check_bash_version
export -f verify_build_env
export -f enforce_script_style
export -f btc_function_name
export -f btc_error_handler
export -f btc_check_git_config
export -f btc_configure_build
export -f btc_run_tests
export -f btc_check_docs
