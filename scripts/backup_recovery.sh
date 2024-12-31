#!/bin/bash

# Backup and Recovery Script for Anya Platform
set -e

# Load environment variables
source ../.env

# Configuration
BACKUP_DIR="/var/backups/anya"
RETENTION_DAYS=30
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="/var/log/anya/backup.log"

# Logging function
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

# Create backup directory if it doesn't exist
mkdir -p "$BACKUP_DIR"

# Backup function
backup() {
    log "Starting backup process..."
    
    # Backup database
    log "Backing up database..."
    pg_dump "$DATABASE_URL" > "$BACKUP_DIR/db_$TIMESTAMP.sql"
    
    # Backup configuration files
    log "Backing up configuration..."
    tar -czf "$BACKUP_DIR/config_$TIMESTAMP.tar.gz" ../config/
    
    # Backup user data
    log "Backing up user data..."
    tar -czf "$BACKUP_DIR/userdata_$TIMESTAMP.tar.gz" ../data/
    
    # Upload to cloud storage if configured
    if [ ! -z "$AWS_ACCESS_KEY_ID" ]; then
        log "Uploading to cloud storage..."
        aws s3 cp "$BACKUP_DIR" "s3://$S3_BUCKET/backups/$TIMESTAMP/" --recursive
    fi
    
    log "Backup completed successfully"
}

# Recovery function
recover() {
    local BACKUP_TIMESTAMP=$1
    
    if [ -z "$BACKUP_TIMESTAMP" ]; then
        log "Error: Backup timestamp required"
        exit 1
    fi
    
    log "Starting recovery process for timestamp: $BACKUP_TIMESTAMP"
    
    # Stop services
    log "Stopping services..."
    systemctl stop anya-services
    
    # Restore database
    log "Restoring database..."
    psql "$DATABASE_URL" < "$BACKUP_DIR/db_$BACKUP_TIMESTAMP.sql"
    
    # Restore configuration
    log "Restoring configuration..."
    tar -xzf "$BACKUP_DIR/config_$BACKUP_TIMESTAMP.tar.gz" -C ../
    
    # Restore user data
    log "Restoring user data..."
    tar -xzf "$BACKUP_DIR/userdata_$BACKUP_TIMESTAMP.tar.gz" -C ../
    
    # Start services
    log "Starting services..."
    systemctl start anya-services
    
    log "Recovery completed successfully"
}

# Cleanup old backups
cleanup() {
    log "Cleaning up old backups..."
    find "$BACKUP_DIR" -type f -mtime +$RETENTION_DAYS -delete
    
    if [ ! -z "$AWS_ACCESS_KEY_ID" ]; then
        log "Cleaning up old backups from cloud storage..."
        aws s3 ls "s3://$S3_BUCKET/backups/" | while read -r line;
        do
            createDate=`echo $line|awk {'print $1" "$2'}`
            createDate=`date -d"$createDate" +%s`
            olderThan=`date -d"-$RETENTION_DAYS days" +%s`
            if [[ $createDate -lt $olderThan ]]
            then 
                aws s3 rm "s3://$S3_BUCKET/backups/$line"
            fi
        done
    fi
    
    log "Cleanup completed"
}

# Parse command line arguments
case "$1" in
    "backup")
        backup
        ;;
    "recover")
        recover "$2"
        ;;
    "cleanup")
        cleanup
        ;;
    *)
        echo "Usage: $0 {backup|recover <timestamp>|cleanup}"
        exit 1
        ;;
esac

exit 0
