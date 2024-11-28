#!/bin/bash

# Health check script for Anya operations
# Monitors system health and writes to a tracked log file

# Configuration
LOG_FILE="../logs/ops_health.log"
METRICS_FILE="../metrics/system_metrics.json"
ALERT_THRESHOLD=90
CHECK_INTERVAL=300  # 5 minutes

# Ensure directories exist
mkdir -p "$(dirname "$LOG_FILE")" "$(dirname "$METRICS_FILE")"

log_message() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] $1" >> "$LOG_FILE"
}

get_system_metrics() {
    # CPU Usage
    local cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}')
    
    # Memory Usage
    local mem_info=$(free -m)
    local mem_usage=$(echo "$mem_info" | awk 'NR==2{printf "%.2f", $3*100/$2}')
    
    # Disk Usage
    local disk_usage=$(df -h / | awk 'NR==2{print $5}' | sed 's/%//')
    
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
    
    if [ $(echo "$cpu_usage > $ALERT_THRESHOLD" | bc -l) -eq 1 ]; then
        log_message "ALERT: High CPU usage: $cpu_usage%"
    fi
    
    if [ $(echo "$mem_usage > $ALERT_THRESHOLD" | bc -l) -eq 1 ]; then
        log_message "ALERT: High memory usage: $mem_usage%"
    fi
    
    if [ $(echo "$disk_usage > $ALERT_THRESHOLD" | bc -l) -eq 1 ]; then
        log_message "ALERT: High disk usage: $disk_usage%"
    fi
}

monitor_services() {
    # Check if core services are running
    local services=("web5_service" "bitcoin_service" "search_service")
    
    for service in "${services[@]}"; do
        if systemctl is-active --quiet "$service" 2>/dev/null; then
            log_message "Service $service is running"
        else
            log_message "ALERT: Service $service is not running"
        fi
    done
}

main() {
    log_message "Starting health check monitoring"
    
    while true; do
        get_system_metrics
        
        # Read metrics from JSON
        local cpu_usage=$(jq -r '.metrics.cpu_usage' "$METRICS_FILE")
        local mem_usage=$(jq -r '.metrics.memory_usage' "$METRICS_FILE")
        local disk_usage=$(jq -r '.metrics.disk_usage' "$METRICS_FILE")
        
        check_thresholds "$cpu_usage" "$mem_usage" "$disk_usage"
        monitor_services
        
        sleep "$CHECK_INTERVAL"
    done
}

# Run main function
main
