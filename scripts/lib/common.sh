#!/bin/bash
# Common utilities and standards for Anya project
# Following Bitcoin Core build and script standards

set -e

# Script locations
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

# Import paths
readonly LIB_DIR="${PROJECT_ROOT}/scripts/lib"
readonly CONFIG_DIR="${PROJECT_ROOT}/config"
readonly LOGS_DIR="${PROJECT_ROOT}/logs"

# Standard exit codes (following Bitcoin Core conventions)
readonly EXIT_SUCCESS=0
readonly EXIT_FAILURE=1
readonly EXIT_BAD_CONFIG=2
readonly EXIT_USER_ERROR=3
readonly EXIT_NOT_SUPPORTED=4
readonly EXIT_ALREADY_RUNNING=5

# Logging levels
declare -A LOG_LEVELS=([ERROR]=0 [WARN]=1 [INFO]=2 [DEBUG]=3)
LOG_LEVEL=${LOG_LEVEL:-INFO}

# Color codes
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# Ensure script is sourced
[[ "${BASH_SOURCE[0]}" != "${0}" ]] || {
    echo "This script must be sourced. Use: source ${BASH_SOURCE[0]}"
    exit "${EXIT_FAILURE}"
}

# Logging functions
log() {
    local level=$1
    shift
    local message=$*
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    if [[ ${LOG_LEVELS[$level]:-0} -le ${LOG_LEVELS[$LOG_LEVEL]:-2} ]]; then
        case $level in
            ERROR) printf "${RED}[%s] ERROR: %s${NC}\n" "$timestamp" "$message" ;;
            WARN)  printf "${YELLOW}[%s] WARN: %s${NC}\n" "$timestamp" "$message" ;;
            INFO)  printf "${GREEN}[%s] INFO: %s${NC}\n" "$timestamp" "$message" ;;
            DEBUG) printf "${BLUE}[%s] DEBUG: %s${NC}\n" "$timestamp" "$message" ;;
        esac
    fi
}

# Configuration management
load_config() {
    local config_file=$1
    if [[ ! -f "$config_file" ]]; then
        log ERROR "Configuration file not found: $config_file"
        return "${EXIT_BAD_CONFIG}"
    fi
    
    # Parse YAML using yq
    if ! command -v yq &> /dev/null; then
        log ERROR "yq is required but not installed"
        return "${EXIT_NOT_SUPPORTED}"
    fi
    
    eval "$(yq e 'to_entries | .[] | select(.value != null) | 
        "export " + (.key | upcase) + "=\"" + (.value | @sh) + "\""' "$config_file")"
}

# Process management
check_process_running() {
    local name=$1
    local pid_file="${LOGS_DIR}/${name}.pid"
    
    if [[ -f "$pid_file" ]]; then
        local pid
        pid=$(cat "$pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            return "${EXIT_ALREADY_RUNNING}"
        fi
    fi
    return "${EXIT_SUCCESS}"
}

save_pid() {
    local name=$1
    local pid=${2:-$$}
    echo "$pid" > "${LOGS_DIR}/${name}.pid"
}

cleanup_pid() {
    local name=$1
    rm -f "${LOGS_DIR}/${name}.pid"
}

# Resource management
get_resource_path() {
    local resource_type=$1
    case $resource_type in
        ml_models)     echo "${PROJECT_ROOT}/ml/models" ;;
        ml_data)       echo "${PROJECT_ROOT}/ml/data" ;;
        embeddings)    echo "${PROJECT_ROOT}/ml/embeddings" ;;
        checkpoints)   echo "${PROJECT_ROOT}/ml/checkpoints" ;;
        temp)          echo "${PROJECT_ROOT}/temp" ;;
        cache)         echo "${PROJECT_ROOT}/cache" ;;
        metrics)       echo "${PROJECT_ROOT}/metrics" ;;
        *)            log ERROR "Unknown resource type: $resource_type"; return 1 ;;
    esac
}

# System checks
check_dependencies() {
    local deps=("$@")
    local missing=()
    
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing+=("$dep")
        fi
    done
    
    if [[ ${#missing[@]} -gt 0 ]]; then
        log ERROR "Missing dependencies: ${missing[*]}"
        return "${EXIT_NOT_SUPPORTED}"
    fi
    return "${EXIT_SUCCESS}"
}

# Version management
get_version() {
    local path=$1
    basename "$path" | grep -oP 'v\d+\.\d+\.\d+'
}

compare_versions() {
    local v1=$1
    local v2=$2
    
    if [[ "$v1" == "$v2" ]]; then
        return 0
    fi
    local IFS=.
    local i ver1=($v1) ver2=($v2)
    for ((i=${#ver1[@]}; i<${#ver2[@]}; i++)); do
        ver1[i]=0
    done
    for ((i=0; i<${#ver1[@]}; i++)); do
        if [[ -z ${ver2[i]} ]]; then
            ver2[i]=0
        fi
        if ((10#${ver1[i]} > 10#${ver2[i]})); then
            return 1
        fi
        if ((10#${ver1[i]} < 10#${ver2[i]})); then
            return 2
        fi
    done
    return 0
}

# Metrics and monitoring
get_system_metrics() {
    local metrics_file=$1
    
    # CPU Usage
    local cpu_usage
    cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}')
    
    # Memory Usage
    local mem_info
    mem_info=$(free -m)
    local mem_usage
    mem_usage=$(echo "$mem_info" | awk 'NR==2{printf "%.2f", $3*100/$2}')
    
    # Disk Usage
    local disk_usage
    disk_usage=$(df -h / | awk 'NR==2{print $5}' | sed 's/%//')
    
    # Write metrics to JSON
    cat > "$metrics_file" << EOF
{
    "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "metrics": {
        "cpu_usage": $cpu_usage,
        "memory_usage": $mem_usage,
        "disk_usage": $disk_usage
    }
}
EOF
}

# Export functions
export -f log
export -f load_config
export -f check_process_running
export -f save_pid
export -f cleanup_pid
export -f get_resource_path
export -f check_dependencies
export -f get_version
export -f compare_versions
export -f get_system_metrics
