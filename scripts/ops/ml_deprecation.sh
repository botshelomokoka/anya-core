#!/bin/bash

# ML-specific deprecation manager for Anya
# Handles ML model versions, training data, checkpoints, and embeddings

source "$(dirname "$0")/deprecation_manager.sh"

# ML paths
ML_ROOT="../ml"
MODELS_DIR="$ML_ROOT/models"
DATA_DIR="$ML_ROOT/data"
EMBEDDINGS_DIR="$ML_ROOT/embeddings"
CHECKPOINTS_DIR="$ML_ROOT/checkpoints"
METRICS_FILE="$ML_ROOT/metrics/performance.json"

check_model_performance() {
    local model_path="$1"
    local min_threshold="$2"
    
    if [[ -f "$METRICS_FILE" ]]; then
        local performance
        performance=$(jq -r --arg model "$(basename "$model_path")" \
            '.models[$model].f1_score' "$METRICS_FILE")
        
        if (( $(echo "$performance >= $min_threshold" | bc -l) )); then
            return 0
        fi
    fi
    return 1
}

check_model_usage() {
    local model_path="$1"
    local min_requests="$2"
    local usage_file="$ML_ROOT/metrics/usage.json"
    
    if [[ -f "$usage_file" ]]; then
        local requests
        requests=$(jq -r --arg model "$(basename "$model_path")" \
            '.models[$model].requests' "$usage_file")
        
        if (( requests >= min_requests )); then
            return 0
        fi
    fi
    return 1
}

check_data_quality() {
    local data_path="$1"
    local min_score="$2"
    local quality_file="$ML_ROOT/metrics/quality.json"
    
    if [[ -f "$quality_file" ]]; then
        local quality
        quality=$(jq -r --arg data "$(basename "$data_path")" \
            '.datasets[$data].quality_score' "$quality_file")
        
        if (( $(echo "$quality >= $min_score" | bc -l) )); then
            return 0
        fi
    fi
    return 1
}

get_model_version() {
    local path="$1"
    basename "$path" | grep -oP 'v\d+\.\d+\.\d+'
}

check_linked_systems() {
    local resource_type="$1"
    local resource_path="$2"
    
    # Check system dependencies
    case "$resource_type" in
        "embeddings")
            if [[ -f "$MODELS_DIR/current_version" ]]; then
                local model_version
                model_version=$(cat "$MODELS_DIR/current_version")
                local emb_version
                emb_version=$(get_model_version "$resource_path")
                
                # Check if embedding version matches model version
                if [[ "$model_version" == "$emb_version" ]]; then
                    return 0
                fi
            fi
            return 1
            ;;
        "checkpoints")
            local model_name
            model_name=$(basename "$(dirname "$resource_path")")
            if [[ -f "$MODELS_DIR/$model_name/best_checkpoint" ]]; then
                local best_checkpoint
                best_checkpoint=$(cat "$MODELS_DIR/$model_name/best_checkpoint")
                
                # Keep if it's the best checkpoint
                if [[ "$(basename "$resource_path")" == "$best_checkpoint" ]]; then
                    return 0
                fi
            fi
            return 1
            ;;
    esac
}

deprecate_ml_resource() {
    local resource_type="$1"
    local resource_path="$2"
    local config_file="../../config/monitoring.yaml"
    
    # Load configuration
    local max_versions
    max_versions=$(yq e ".deprecation.system_requirements.ml.model_versions.max_kept" "$config_file")
    local max_checkpoints
    max_checkpoints=$(yq e ".deprecation.system_requirements.ml.checkpoints.max_per_model" "$config_file")
    
    case "$resource_type" in
        "models")
            # Keep top performing models
            if ! check_model_performance "$resource_path" 0.85 || \
               ! check_model_usage "$resource_path" 100; then
                # Archive and remove
                local archive_dir="$MODELS_DIR/archive"
                mkdir -p "$archive_dir"
                tar -czf "$archive_dir/$(basename "$resource_path")_$(date +%Y%m%d).tar.gz" \
                    -C "$(dirname "$resource_path")" "$(basename "$resource_path")"
                rm -rf "$resource_path"
                log_message "Deprecated model: $resource_path"
            fi
            
            # Cleanup old versions beyond max_versions
            find "$MODELS_DIR" -maxdepth 1 -type d -name "v*" | \
                sort -V | head -n -"$max_versions" | xargs -r rm -rf
            ;;
            
        "data")
            if ! check_data_quality "$resource_path" 0.8; then
                local archive_dir="$DATA_DIR/archive"
                mkdir -p "$archive_dir"
                tar -czf "$archive_dir/$(basename "$resource_path")_$(date +%Y%m%d).tar.gz" \
                    -C "$(dirname "$resource_path")" "$(basename "$resource_path")"
                rm -rf "$resource_path"
                log_message "Deprecated training data: $resource_path"
            fi
            ;;
            
        "embeddings")
            if ! check_linked_systems "embeddings" "$resource_path"; then
                local archive_dir="$EMBEDDINGS_DIR/archive"
                mkdir -p "$archive_dir"
                tar -czf "$archive_dir/$(basename "$resource_path")_$(date +%Y%m%d).tar.gz" \
                    -C "$(dirname "$resource_path")" "$(basename "$resource_path")"
                rm -rf "$resource_path"
                log_message "Deprecated embeddings: $resource_path"
            fi
            ;;
            
        "checkpoints")
            if ! check_linked_systems "checkpoints" "$resource_path"; then
                local model_name
                model_name=$(basename "$(dirname "$resource_path")")
                local checkpoint_count
                checkpoint_count=$(find "$(dirname "$resource_path")" -maxdepth 1 -type f | wc -l)
                
                if (( checkpoint_count > max_checkpoints )); then
                    rm -f "$resource_path"
                    log_message "Deprecated checkpoint: $resource_path"
                fi
            fi
            ;;
    esac
}

check_system_requirements() {
    local config_file="../../config/monitoring.yaml"
    
    # Check system performance thresholds
    local cpu_usage
    cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}')
    local mem_usage
    mem_usage=$(free | grep Mem | awk '{print $3/$2 * 100.0}')
    local disk_usage
    disk_usage=$(df -h / | awk 'NR==2{print $5}' | sed 's/%//')
    
    local cpu_threshold
    cpu_threshold=$(yq e ".deprecation.system_requirements.performance.cpu_threshold" "$config_file")
    local mem_threshold
    mem_threshold=$(yq e ".deprecation.system_requirements.performance.memory_threshold" "$config_file")
    local disk_threshold
    disk_threshold=$(yq e ".deprecation.system_requirements.performance.disk_threshold" "$config_file")
    
    if (( $(echo "$cpu_usage > $cpu_threshold" | bc -l) )) || \
       (( $(echo "$mem_usage > $mem_threshold" | bc -l) )) || \
       (( $(echo "$disk_usage > $disk_threshold" | bc -l) )); then
        return 0  # System requirements exceeded, need aggressive cleanup
    fi
    return 1
}

main() {
    # Check if system requirements trigger aggressive cleanup
    if check_system_requirements; then
        log_message "System requirements exceeded - performing aggressive cleanup"
        
        # Aggressive cleanup strategy
        find "$MODELS_DIR" -type d -name "v*" | while read -r model; do
            deprecate_ml_resource "models" "$model"
        done
        
        find "$DATA_DIR" -type f -mtime +30 | while read -r data; do
            deprecate_ml_resource "data" "$data"
        done
        
        find "$EMBEDDINGS_DIR" -type d -name "v*" | while read -r emb; do
            deprecate_ml_resource "embeddings" "$emb"
        done
        
        find "$CHECKPOINTS_DIR" -type f | while read -r checkpoint; do
            deprecate_ml_resource "checkpoints" "$checkpoint"
        done
    else
        # Normal cleanup based on individual resource requirements
        find "$ML_ROOT" -type f -o -type d | while read -r resource; do
            case "$resource" in
                */models/*)
                    deprecate_ml_resource "models" "$resource"
                    ;;
                */data/*)
                    deprecate_ml_resource "data" "$resource"
                    ;;
                */embeddings/*)
                    deprecate_ml_resource "embeddings" "$resource"
                    ;;
                */checkpoints/*)
                    deprecate_ml_resource "checkpoints" "$resource"
                    ;;
            esac
        done
    fi
}

# Run main function if not sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main
fi
