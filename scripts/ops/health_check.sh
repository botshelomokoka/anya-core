#!/bin/bash
set -euo pipefail

# Health check script for Anya operations
# Monitors system health and writes to a tracked log file

# Import common utilities
# shellcheck source=../lib/common.sh
source "$(dirname "${BASH_SOURCE[0]}")/../lib/common.sh"

# Import managers
# shellcheck source=./log_manager.sh
source "$(dirname "${BASH_SOURCE[0]}")/log_manager.sh"
# shellcheck source=./deprecation_manager.sh
source "$(dirname "${BASH_SOURCE[0]}")/deprecation_manager.sh"

# Configuration
readonly LOG_FILE="../logs/ops_health.log"
readonly METRICS_FILE="../metrics/system_metrics.json"
readonly ALERT_THRESHOLD=90
readonly CHECK_INTERVAL=300  # 5 minutes

# Ensure directories exist
mkdir -p "$(dirname "$LOG_FILE")" "$(dirname "$METRICS_FILE")"

log_message() {
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] $1" >> "$LOG_FILE"
    
    # Check if log management is needed
    if [[ -f "$LOG_FILE" ]]; then
        rotate_logs "$LOG_FILE"
        deprecate_logs
    fi
}

get_system_metrics() {
    local cpu_usage mem_info mem_usage disk_usage
    
    # CPU Usage
    cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}')
    
    # Memory Usage
    mem_info=$(free -m)
    mem_usage=$(echo "$mem_info" | awk 'NR==2{printf "%.2f", $3*100/$2}')
    
    # Disk Usage
    disk_usage=$(df -h / | awk 'NR==2{print $5}' | sed 's/%//')
    
    # Write metrics to JSON file
    cat > "$METRICS_FILE" << EOF
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

check_thresholds() {
    local cpu_usage=$1
    local mem_usage=$2
    local disk_usage=$3
    
    if (( $(echo "$cpu_usage > $ALERT_THRESHOLD" | bc -l) )); then
        log_message "ALERT: High CPU usage: $cpu_usage%"
    fi
    
    if (( $(echo "$mem_usage > $ALERT_THRESHOLD" | bc -l) )); then
        log_message "ALERT: High memory usage: $mem_usage%"
    fi
    
    if (( $(echo "$disk_usage > $ALERT_THRESHOLD" | bc -l) )); then
        log_message "ALERT: High disk usage: $disk_usage%"
    fi
}

monitor_services() {
    local services=("web5_service" "bitcoin_service" "search_service")
    local service
    
    for service in "${services[@]}"; do
        if systemctl is-active --quiet "$service" 2>/dev/null; then
            log_message "Service $service is running"
        else
            log_message "ALERT: Service $service is not running"
        fi
    done
}

check_deprecation_schedule() {
    if [[ $(date +%H:%M) == "02:00" ]]; then
        run_deprecation  # Run deprecation manager main function
    fi
}

cleanup() {
    log_message "Stopping health check monitoring"
    rm -f "$METRICS_FILE"
}

main() {
    log_message "Starting health check monitoring"
    trap cleanup EXIT
    
    while true; do
        get_system_metrics
        
        # Read metrics from JSON
        local cpu_usage mem_usage disk_usage
        cpu_usage=$(jq -r '.metrics.cpu_usage' "$METRICS_FILE")
        mem_usage=$(jq -r '.metrics.memory_usage' "$METRICS_FILE")
        disk_usage=$(jq -r '.metrics.disk_usage' "$METRICS_FILE")
        
        check_thresholds "$cpu_usage" "$mem_usage" "$disk_usage"
        monitor_services
        check_deprecation_schedule
        
        sleep "$CHECK_INTERVAL"
    done
}

# Run main function
main
