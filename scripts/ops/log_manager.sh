#!/bin/bash

# Log management script for Anya operations
# Handles log rotation, backup, and deprecation based on requirements

source "$(dirname "$0")/../../config/monitoring.yaml"

# Configuration
CONFIG_FILE="../../config/monitoring.yaml"
LOG_DIR="../logs"
BACKUP_DIR="../backups/logs"
METRICS_FILE="../metrics/system_metrics.json"

# Ensure directories exist
mkdir -p "$LOG_DIR" "$BACKUP_DIR"

check_requirements() {
    local all_met=true
    
    # Check backup existence
    if [[ ! -d "$BACKUP_DIR" ]] || [[ -z "$(ls -A "$BACKUP_DIR")" ]]; then
        echo "Requirement not met: No backup exists"
        all_met=false
    fi
    
    # Check metrics export status
    if [[ ! -f "$METRICS_FILE" ]] || [[ ! -s "$METRICS_FILE" ]]; then
        echo "Requirement not met: Metrics not exported"
        all_met=false
    fi
    
    # Check minimum retention period
    local newest_log
    newest_log=$(find "$LOG_DIR" -type f -name "*.log" -printf '%T@ %p\n' | sort -n | tail -1 | cut -d' ' -f2-)
    if [[ -n "$newest_log" ]]; then
        local file_age
        file_age=$(( ($(date +%s) - $(date +%s -r "$newest_log")) / 86400 ))
        if (( file_age < 7 )); then
            echo "Requirement not met: Minimum retention period not reached"
            all_met=false
        fi
    fi
    
    # Check disk space
    local disk_free
    disk_free=$(df -h "$LOG_DIR" | awk 'NR==2 {print $5}' | sed 's/%//')
    if (( disk_free < 20 )); then
        echo "Warning: Low disk space, forcing log cleanup regardless of requirements"
        return 0
    fi
    
    $all_met
}

rotate_logs() {
    local log_file="$1"
    local max_size=104857600  # 100MB in bytes
    
    if [[ -f "$log_file" ]]; then
        local file_size
        file_size=$(stat -f%z "$log_file" 2>/dev/null || stat -c%s "$log_file")
        
        if (( file_size > max_size )); then
            local timestamp
            timestamp=$(date +%Y%m%d_%H%M%S)
            local rotated_file="${log_file}.${timestamp}"
            
            mv "$log_file" "$rotated_file"
            gzip "$rotated_file"
            
            # Create new empty log file
            touch "$log_file"
            
            # Remove old rotated logs if we have too many
            local max_rotated_files=5
            find "$LOG_DIR" -name "${log_file##*/}.*" -type f | sort -r | tail -n +$((max_rotated_files + 1)) | xargs rm -f
        fi
    fi
}

backup_logs() {
    local timestamp
    timestamp=$(date +%Y%m%d_%H%M%S)
    local backup_file="$BACKUP_DIR/logs_backup_${timestamp}.tar.gz"
    
    # Create backup
    tar -czf "$backup_file" -C "$LOG_DIR" .
    
    # Manage backup retention
    cleanup_backups "daily" 7
    cleanup_backups "weekly" 4
    cleanup_backups "monthly" 3
}

cleanup_backups() {
    local retention_type="$1"
    local retention_count="$2"
    
    case "$retention_type" in
        "daily")
            find "$BACKUP_DIR" -name "logs_backup_*.tar.gz" -type f -mtime +${retention_count} -delete
            ;;
        "weekly")
            find "$BACKUP_DIR" -name "logs_backup_*.tar.gz" -type f -mtime +$((retention_count * 7)) -delete
            ;;
        "monthly")
            find "$BACKUP_DIR" -name "logs_backup_*.tar.gz" -type f -mtime +$((retention_count * 30)) -delete
            ;;
    esac
}

deprecate_logs() {
    if ! check_requirements; then
        echo "Cannot deprecate logs - requirements not met"
        return 1
    fi
    
    local max_age_days=30
    local max_size_mb=500
    
    # Remove old logs
    find "$LOG_DIR" -name "*.log" -type f -mtime +${max_age_days} -delete
    
    # Check total log size and remove oldest if exceeds max
    local total_size
    total_size=$(du -sm "$LOG_DIR" | cut -f1)
    
    if (( total_size > max_size_mb )); then
        echo "Total log size ($total_size MB) exceeds maximum ($max_size_mb MB). Removing oldest logs..."
        while (( $(du -sm "$LOG_DIR" | cut -f1) > max_size_mb )); do
            local oldest_log
            oldest_log=$(find "$LOG_DIR" -name "*.log" -type f -printf '%T@ %p\n' | sort -n | head -1 | cut -d' ' -f2-)
            if [[ -n "$oldest_log" ]]; then
                rm -f "$oldest_log"
            else
                break
            fi
        done
    fi
}

main() {
    # Rotate current logs
    find "$LOG_DIR" -name "*.log" -type f | while read -r log_file; do
        rotate_logs "$log_file"
    done
    
    # Backup logs daily
    if [[ $(date +%H:%M) == "00:00" ]]; then
        backup_logs
    fi
    
    # Check and deprecate logs
    deprecate_logs
}

# Run main function
main
