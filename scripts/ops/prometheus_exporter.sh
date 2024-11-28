#!/bin/bash

# Prometheus metrics exporter for Anya
# Exposes system metrics in Prometheus format

# Configuration
METRICS_PORT=9090
METRICS_PATH="/metrics"
METRICS_FILE="../metrics/system_metrics.json"

generate_prometheus_metrics() {
    local metrics_json=$(cat "$METRICS_FILE")
    
    # CPU Usage
    echo "# HELP anya_cpu_usage Current CPU usage percentage"
    echo "# TYPE anya_cpu_usage gauge"
    echo "anya_cpu_usage $(jq -r '.metrics.cpu_usage' <<< "$metrics_json")"
    
    # Memory Usage
    echo "# HELP anya_memory_usage Current memory usage percentage"
    echo "# TYPE anya_memory_usage gauge"
    echo "anya_memory_usage $(jq -r '.metrics.memory_usage' <<< "$metrics_json")"
    
    # Disk Usage
    echo "# HELP anya_disk_usage Current disk usage percentage"
    echo "# TYPE anya_disk_usage gauge"
    echo "anya_disk_usage $(jq -r '.metrics.disk_usage' <<< "$metrics_json")"
    
    # Service Status
    echo "# HELP anya_service_status Status of Anya services (1=up, 0=down)"
    echo "# TYPE anya_service_status gauge"
    
    local services=("web5_service" "bitcoin_service" "search_service")
    for service in "${services[@]}"; do
        if systemctl is-active --quiet "$service" 2>/dev/null; then
            echo "anya_service_status{service=\"$service\"} 1"
        else
            echo "anya_service_status{service=\"$service\"} 0"
        fi
    done
}

# Start a simple HTTP server to expose metrics
python3 -m http.server "$METRICS_PORT" &
SERVER_PID=$!

cleanup() {
    kill $SERVER_PID
    exit 0
}

trap cleanup SIGINT SIGTERM

# Main loop to update metrics
while true; do
    generate_prometheus_metrics > /tmp/anya_metrics
    sleep 60
done
