#!/bin/bash
# Module management system for Anya project
# Handles script dependencies and cross-module integration

# Import standards
# shellcheck source=./common.sh
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
# shellcheck source=./bitcoin_standards.sh
source "$(dirname "${BASH_SOURCE[0]}")/bitcoin_standards.sh"

# Module registry
declare -A MODULE_REGISTRY
declare -A MODULE_DEPENDENCIES
declare -A MODULE_VERSIONS

# Module states
readonly MODULE_STATE_INACTIVE=0
readonly MODULE_STATE_ACTIVE=1
readonly MODULE_STATE_ERROR=2

# Initialize module system
init_module_system() {
    log INFO "Initializing module system"
    
    # Load module configuration
    load_config "${CONFIG_DIR}/modules.yaml" || {
        log ERROR "Failed to load module configuration"
        return "${EXIT_BAD_CONFIG}"
    }
    
    # Create module directories
    mkdir -p "${PROJECT_ROOT}/modules"
}

# Register a module
register_module() {
    local name=$1
    local path=$2
    local version=$3
    local dependencies=("${@:4}")
    
    # Validate module
    if [[ -z "$name" || -z "$path" || -z "$version" ]]; then
        log ERROR "Invalid module registration: $name"
        return "${EXIT_BAD_CONFIG}"
    }
    
    # Check if module exists
    if [[ -n "${MODULE_REGISTRY[$name]}" ]]; then
        log WARN "Module $name already registered"
        return "${EXIT_ALREADY_RUNNING}"
    }
    
    # Register module
    MODULE_REGISTRY[$name]=$path
    MODULE_VERSIONS[$name]=$version
    MODULE_DEPENDENCIES[$name]="${dependencies[*]}"
    
    log INFO "Registered module: $name (v$version)"
}

# Load module dependencies
load_module_dependencies() {
    local name=$1
    local deps="${MODULE_DEPENDENCIES[$name]}"
    
    if [[ -n "$deps" ]]; then
        for dep in $deps; do
            if [[ -z "${MODULE_REGISTRY[$dep]}" ]]; then
                log ERROR "Missing dependency: $dep for module $name"
                return "${EXIT_FAILURE}"
            fi
            
            # Load dependency if not already loaded
            if [[ ${MODULE_STATE[$dep]:-$MODULE_STATE_INACTIVE} == "$MODULE_STATE_INACTIVE" ]]; then
                load_module "$dep"
            fi
        done
    fi
}

# Load a module
load_module() {
    local name=$1
    local path="${MODULE_REGISTRY[$name]}"
    
    if [[ -z "$path" ]]; then
        log ERROR "Module not found: $name"
        return "${EXIT_FAILURE}"
    }
    
    # Load dependencies first
    load_module_dependencies "$name" || return $?
    
    # Source module file
    if [[ -f "$path" ]]; then
        # shellcheck source=/dev/null
        source "$path" || {
            log ERROR "Failed to load module: $name"
            MODULE_STATE[$name]=$MODULE_STATE_ERROR
            return "${EXIT_FAILURE}"
        }
        
        MODULE_STATE[$name]=$MODULE_STATE_ACTIVE
        log INFO "Loaded module: $name"
    else
        log ERROR "Module file not found: $path"
        return "${EXIT_FAILURE}"
    fi
}

# Check module compatibility
check_module_compatibility() {
    local name=$1
    local version="${MODULE_VERSIONS[$name]}"
    local deps="${MODULE_DEPENDENCIES[$name]}"
    
    if [[ -n "$deps" ]]; then
        for dep in $deps; do
            local dep_version="${MODULE_VERSIONS[$dep]}"
            if ! check_version_compatibility "$version" "$dep_version"; then
                log ERROR "Incompatible module versions: $name ($version) -> $dep ($dep_version)"
                return "${EXIT_NOT_SUPPORTED}"
            fi
        done
    fi
}

# Check version compatibility
check_version_compatibility() {
    local v1=$1
    local v2=$2
    compare_versions "$v1" "$v2"
    return $?
}

# Get module status
get_module_status() {
    local name=$1
    local state=${MODULE_STATE[$name]:-$MODULE_STATE_INACTIVE}
    
    case $state in
        $MODULE_STATE_INACTIVE) echo "inactive" ;;
        $MODULE_STATE_ACTIVE)   echo "active" ;;
        $MODULE_STATE_ERROR)    echo "error" ;;
        *)                      echo "unknown" ;;
    esac
}

# Unload a module
unload_module() {
    local name=$1
    
    if [[ ${MODULE_STATE[$name]:-$MODULE_STATE_INACTIVE} == "$MODULE_STATE_ACTIVE" ]]; then
        # Check if other modules depend on this one
        for module in "${!MODULE_DEPENDENCIES[@]}"; do
            if [[ "${MODULE_DEPENDENCIES[$module]}" == *"$name"* ]]; then
                log ERROR "Cannot unload module $name: required by $module"
                return "${EXIT_FAILURE}"
            fi
        done
        
        MODULE_STATE[$name]=$MODULE_STATE_INACTIVE
        log INFO "Unloaded module: $name"
    fi
}

# Export functions
export -f init_module_system
export -f register_module
export -f load_module
export -f load_module_dependencies
export -f check_module_compatibility
export -f get_module_status
export -f unload_module
