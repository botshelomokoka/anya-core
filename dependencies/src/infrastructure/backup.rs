use std::path::Path;
use tokio::process::Command;
use chrono::Utc;
use crate::infrastructure::error::DBError;

pub struct BackupManager {
    db_url: String,
    backup_dir: PathBuf,
}

impl BackupManager {
    pub fn new(db_url: String, backup_dir: PathBuf) -> Self {
        Self { db_url, backup_dir }
    }

    pub async fn create_backup(&self) -> Result<PathBuf, DBError> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("backup_{}.sql", timestamp);
        let backup_path = self.backup_dir.join(filename);

        let output = Command::new("pg_dump")
            .arg(&self.db_url)
            .arg("-F")
            .arg("c")
            .arg("-f")
            .arg(&backup_path)
            .output()
            .await
            .map_err(|e| DBError::Backup(e.to_string()))?;

        if !output.status.success() {
            return Err(DBError::Backup(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        Ok(backup_path)
    }

    pub async fn restore_backup(&self, backup_path: &Path) -> Result<(), DBError> {
        let output = Command::new("pg_restore")
            .arg("-d")
            .arg(&self.db_url)
            .arg(backup_path)
            .output()
            .await
            .map_err(|e| DBError::Restore(e.to_string()))?;

        if !output.status.success() {
            return Err(DBError::Restore(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        Ok(())
    }
}
