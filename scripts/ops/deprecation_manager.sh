#!/bin/bash

# Deprecation manager for Anya operations
# Handles deprecation of various system resources and temporary files

# Configuration
CONFIG_FILE="../../config/monitoring.yaml"
TEMP_DIR="../temp"
CACHE_DIR="../cache"
METRICS_ARCHIVE="../metrics/archive"
SEARCH_CACHE="../web5/advanced_search/cache"

# Import common functions
source "$(dirname "$0")/log_manager.sh"

check_deprecation_requirements() {
    local resource_type="$1"
    local resource_path="$2"
    
    case "$resource_type" in
        "temp")
            # Temp files can be deprecated if older than 24 hours
            return 0
            ;;
        "cache")
            # Check if cache is not actively used
            if ! check_cache_usage "$resource_path"; then
                return 0
            fi
            return 1
            ;;
        "metrics")
            # Check if metrics are exported to long-term storage
            if check_metrics_exported "$resource_path"; then
                return 0
            fi
            return 1
            ;;
        "search_index")
            # Check if search index is not in use and backed up
            if check_search_index_status "$resource_path"; then
                return 0
            fi
            return 1
            ;;
        *)
            echo "Unknown resource type: $resource_type"
            return 1
            ;;
    esac
}

check_cache_usage() {
    local cache_path="$1"
    local usage_threshold=3600  # 1 hour in seconds
    
    # Check last access time
    local last_access
    last_access=$(stat -c %X "$cache_path" 2>/dev/null || stat -f %a "$cache_path")
    local current_time
    current_time=$(date +%s)
    
    if (( current_time - last_access > usage_threshold )); then
        return 0  # Can be deprecated
    fi
    return 1  # Still in use
}

check_metrics_exported() {
    local metrics_path="$1"
    local archive_path="${metrics_path%/*}/archive"
    
    # Check if metrics are archived
    if [[ -f "$archive_path/$(date +%Y%m).tar.gz" ]]; then
        return 0  # Metrics are archived
    fi
    return 1  # Not archived
}

check_search_index_status() {
    local index_path="$1"
    local backup_path="${index_path%/*}/backup"
    
    # Check if index is backed up and not in use
    if [[ -f "$backup_path/$(date +%Y%m%d)_index.bak" ]] && \
       ! fuser "$index_path" >/dev/null 2>&1; then
        return 0  # Can be deprecated
    fi
    return 1  # In use or not backed up
}

deprecate_resource() {
    local resource_type="$1"
    local resource_path="$2"
    local max_age="$3"
    
    if ! check_deprecation_requirements "$resource_type" "$resource_path"; then
        log_message "Cannot deprecate $resource_type at $resource_path - requirements not met"
        return 1
    fi
    
    case "$resource_type" in
        "temp")
            find "$resource_path" -type f -mtime +"$max_age" -delete
            ;;
        "cache")
            # Archive important cache entries before deletion
            local archive_dir="${resource_path%/*}/archive"
            mkdir -p "$archive_dir"
            find "$resource_path" -type f -mtime +"$max_age" -exec tar -czf \
                "$archive_dir/cache_$(date +%Y%m%d).tar.gz" {} + -delete
            ;;
        "metrics")
            # Compress old metrics and move to archive
            local archive_dir="${resource_path%/*}/archive"
            mkdir -p "$archive_dir"
            find "$resource_path" -type f -mtime +"$max_age" -name "*.json" \
                -exec tar -czf "$archive_dir/metrics_$(date +%Y%m).tar.gz" {} + -delete
            ;;
        "search_index")
            # Backup and remove old search indices
            local backup_dir="${resource_path%/*}/backup"
            mkdir -p "$backup_dir"
            find "$resource_path" -type f -mtime +"$max_age" \
                -exec cp {} "$backup_dir/$(date +%Y%m%d)_index.bak" \; -delete
            ;;
    esac
    
    log_message "Deprecated $resource_type resources older than $max_age days from $resource_path"
}

cleanup_deprecated() {
    local resource_type="$1"
    local archive_path="$2"
    local retention_days="$3"
    
    if [[ -d "$archive_path" ]]; then
        find "$archive_path" -type f -mtime +"$retention_days" -delete
        log_message "Cleaned up deprecated $resource_type archives older than $retention_days days"
    fi
}

main() {
    # Deprecate temporary files (24 hours)
    deprecate_resource "temp" "$TEMP_DIR" 1
    
    # Deprecate cache files (7 days)
    deprecate_resource "cache" "$CACHE_DIR" 7
    
    # Deprecate old metrics (30 days)
    deprecate_resource "metrics" "$METRICS_ARCHIVE" 30
    
    # Deprecate old search indices (14 days)
    deprecate_resource "search_index" "$SEARCH_CACHE" 14
    
    # Cleanup deprecated archives
    cleanup_deprecated "cache" "${CACHE_DIR%/*}/archive" 90
    cleanup_deprecated "metrics" "${METRICS_ARCHIVE%/*}/archive" 365
    cleanup_deprecated "search_index" "${SEARCH_CACHE%/*}/backup" 180
}

# Run main function if not sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main
fi
