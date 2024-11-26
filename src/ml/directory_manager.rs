use super::file_tracker::{FileTracker, FileCategory, FileMetadata};
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use tokio::fs;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryStructure {
    root: PathBuf,
    core_modules: Vec<ModuleInfo>,
    feature_modules: Vec<ModuleInfo>,
    enterprise_modules: Vec<ModuleInfo>,
    test_modules: Vec<ModuleInfo>,
    config_files: Vec<FileInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleInfo {
    name: String,
    path: PathBuf,
    dependencies: Vec<String>,
    files: Vec<FileInfo>,
    importance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    name: String,
    path: PathBuf,
    category: FileCategory,
    last_modified: chrono::DateTime<chrono::Utc>,
}

pub struct DirectoryManager {
    file_tracker: FileTracker,
    structure: DirectoryStructure,
}

impl DirectoryManager {
    pub async fn new() -> Result<Self> {
        let file_tracker = FileTracker::new().await;
        let structure = DirectoryStructure {
            root: PathBuf::from("anya"),
            core_modules: Vec::new(),
            feature_modules: Vec::new(),
            enterprise_modules: Vec::new(),
            test_modules: Vec::new(),
            config_files: Vec::new(),
        };

        Ok(Self {
            file_tracker,
            structure,
        })
    }

    pub async fn scan_directory(&mut self) -> Result<()> {
        // Scan core modules
        self.scan_core_modules().await?;
        
        // Scan feature modules
        self.scan_feature_modules().await?;
        
        // Scan enterprise modules
        self.scan_enterprise_modules().await?;
        
        // Scan tests
        self.scan_test_modules().await?;
        
        // Scan config files
        self.scan_config_files().await?;

        Ok(())
    }

    async fn scan_core_modules(&mut self) -> Result<()> {
        let core_path = self.structure.root.join("src");
        let core_modules = [
            "blockchain",
            "identity",
            "ml_logic",
            "network",
            "secure_storage",
            "secrets",
        ];

        for module in core_modules.iter() {
            let module_path = core_path.join(module);
            if module_path.exists() {
                let module_info = self.analyze_module(&module_path).await?;
                self.structure.core_modules.push(module_info);
            }
        }

        Ok(())
    }

    async fn scan_feature_modules(&mut self) -> Result<()> {
        let feature_path = self.structure.root.join("src");
        let feature_modules = [
            "smart_contracts",
            "interoperability",
            "privacy",
            "federated_learning",
        ];

        for module in feature_modules.iter() {
            let module_path = feature_path.join(module);
            if module_path.exists() {
                let module_info = self.analyze_module(&module_path).await?;
                self.structure.feature_modules.push(module_info);
            }
        }

        Ok(())
    }

    async fn scan_enterprise_modules(&mut self) -> Result<()> {
        let enterprise_path = self.structure.root.join("anya-enterprise/src");
        if enterprise_path.exists() {
            let entries = fs::read_dir(&enterprise_path).await?;
            tokio::pin!(entries);

            while let Some(entry) = entries.next_entry().await? {
                let module_info = self.analyze_module(&entry.path()).await?;
                self.structure.enterprise_modules.push(module_info);
            }
        }

        Ok(())
    }

    async fn analyze_module(&self, path: &Path) -> Result<ModuleInfo> {
        let mut files = Vec::new();
        let mut dependencies = Vec::new();
        let mut total_importance = 0.0;
        let mut file_count = 0;

        let entries = fs::read_dir(path).await?;
        tokio::pin!(entries);

        while let Some(entry) = entries.next_entry().await? {
            let metadata = self.file_tracker.track_file(&entry.path()).await?;
            
            files.push(FileInfo {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path(),
                category: metadata.category,
                last_modified: metadata.last_modified,
            });

            dependencies.extend(self.extract_dependencies(&entry.path()).await?);
            total_importance += metadata.importance_score;
            file_count += 1;
        }

        Ok(ModuleInfo {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            path: path.to_path_buf(),
            dependencies: dependencies.into_iter().collect(),
            files,
            importance: total_importance / file_count as f64,
        })
    }

    async fn extract_dependencies(&self, file_path: &Path) -> Result<Vec<String>> {
        let content = fs::read_to_string(file_path).await?;
        let mut deps = Vec::new();

        // Extract use statements
        for line in content.lines() {
            if line.trim().starts_with("use") {
                let dep = line.split("use").nth(1).unwrap_or("").trim()
                    .split("::").next().unwrap_or("")
                    .trim_matches(|c| c == '{' || c == '}' || c == ';')
                    .to_string();
                if !dep.is_empty() {
                    deps.push(dep);
                }
            }
        }

        Ok(deps)
    }

    pub async fn get_module_info(&self, module_name: &str) -> Result<Option<&ModuleInfo>> {
        // Search in all module categories
        for module in &self.structure.core_modules {
            if module.name == module_name {
                return Ok(Some(module));
            }
        }
        // ... check other module categories

        Ok(None)
    }

    pub async fn get_file_info(&self, path: &Path) -> Result<Option<&FileInfo>> {
        for module in &self.structure.core_modules {
            for file in &module.files {
                if file.path == path {
                    return Ok(Some(file));
                }
            }
        }
        // ... check other module categories

        Ok(None)
    }

    pub async fn generate_report(&self) -> Result<String> {
        let report = serde_json::to_string_pretty(&self.structure)?;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_directory_manager() {
        let mut manager = DirectoryManager::new().await.unwrap();
        manager.scan_directory().await.unwrap();
        
        let report = manager.generate_report().await.unwrap();
        assert!(!report.is_empty());
    }
}
