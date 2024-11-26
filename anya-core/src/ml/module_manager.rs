use super::file_tracker::{FileTracker, FileCategory};
use anyhow::Result;
use tokio::fs;
use std::path::Path;

pub struct ModuleManager {
    file_tracker: FileTracker,
}

impl ModuleManager {
    pub async fn new() -> Self {
        Self {
            file_tracker: FileTracker::new().await,
        }
    }

    pub async fn reorganize_modules(&self) -> Result<()> {
        // Move anya-core modules to src
        self.move_core_modules().await?;
        
        // Update file tracking
        self.update_file_tracking().await?;
        
        Ok(())
    }

    async fn move_core_modules(&self) -> Result<()> {
        let core_path = Path::new("anya-core/src");
        let target_path = Path::new("src");

        let mut entries = fs::read_dir(core_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let source = entry.path();
            let target = target_path.join(source.file_name().unwrap());
            
            if source.is_dir() {
                fs::create_dir_all(&target).await?;
                copy_dir_all(&source, &target).await?;
            } else {
                fs::copy(&source, &target).await?;
            }
        }

        Ok(())
    }

    async fn update_file_tracking(&self) -> Result<()> {
        let mut entries = fs::read_dir("src").await?;
        while let Some(entry) = entries.next_entry().await? {
            self.file_tracker.track_file(&entry.path()).await?;
        }
        Ok(())
    }
}

async fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(&dst).await?;
    let mut entries = fs::read_dir(src).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let ty = entry.file_type().await?;
        let source = entry.path();
        let target = dst.join(source.file_name().unwrap());

        if ty.is_dir() {
            copy_dir_all(&source, &target).await?;
        } else {
            fs::copy(&source, &target).await?;
        }
    }
    
    Ok(())
}
