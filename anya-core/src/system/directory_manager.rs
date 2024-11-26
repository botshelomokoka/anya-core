use std::path::{Path, PathBuf};
use tokio::fs::{self, ReadDir};
use notify::{Watcher, RecursiveMode, Result as NotifyResult};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SystemLayer {
    #[serde(rename = "layer1")]
    Layer1Core,  // Immutable Bitcoin core principles
    #[serde(rename = "layer2")]
    Layer2Extensions,  // Lightning, DLC, etc.
    #[serde(rename = "layer3")]
    Layer3Applications, // ML and application logic
}

#[derive(Debug)]
pub struct DirectoryManager {
    root_path: PathBuf,
    watcher: notify::RecommendedWatcher,
    ml_manager: Arc<Mutex<crate::ml::MLManager>>,
    layer_map: HashMap<PathBuf, SystemLayer>,
}

impl DirectoryManager {
    pub async fn new(root_path: PathBuf, ml_manager: Arc<Mutex<crate::ml::MLManager>>) -> Result<Self> {
        let mut watcher = notify::recommended_watcher(Self::handle_change)?;
        watcher.watch(&root_path, RecursiveMode::Recursive)?;
        
        let mut manager = Self {
            root_path,
            watcher,
            ml_manager,
            layer_map: HashMap::new(),
        };
        
        manager.initialize_layer_mapping().await?;
        Ok(manager)
    }

    async fn initialize_layer_mapping(&mut self) -> Result<()> {
        // Layer 1 core paths (immutable)
        self.layer_map.insert(
            self.root_path.join("src/bitcoin"),
            SystemLayer::Layer1Core
        );
        self.layer_map.insert(
            self.root_path.join("src/consensus"),
            SystemLayer::Layer1Core
        );

        // Scan and categorize other directories
        self.scan_and_categorize_directories().await?;
        Ok(())
    }

    async fn scan_and_categorize_directories(&mut self) -> Result<()> {
        let mut entries = fs::read_dir(&self.root_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                let layer = self.detect_layer(&path).await?;
                self.layer_map.insert(path, layer);
            }
        }
        Ok(())
    }

    async fn detect_layer(&self, path: &Path) -> Result<SystemLayer> {
        // Check for Layer 1 core markers
        if self.is_core_component(path).await? {
            return Ok(SystemLayer::Layer1Core);
        }

        // Check for Layer 2 extensions
        if self.is_layer2_extension(path).await? {
            return Ok(SystemLayer::Layer2Extensions);
        }

        // Default to Layer 3
        Ok(SystemLayer::Layer3Applications)
    }

    async fn is_core_component(&self, path: &Path) -> Result<bool> {
        // Check for core Bitcoin markers
        let is_core = path.to_str()
            .map(|p| p.contains("bitcoin") || p.contains("consensus"))
            .unwrap_or(false);

        if is_core {
            // Verify core principles are maintained
            self.verify_core_principles(path).await?;
        }

        Ok(is_core)
    }

    async fn verify_core_principles(&self, path: &Path) -> Result<()> {
        // Read and verify core principles
        let principles = self.read_core_principles(path).await?;
        
        // Ensure core principles haven't been modified
        if !self.validate_core_principles(&principles)? {
            anyhow::bail!("Core principles have been modified");
        }
        
        Ok(())
    }

    fn handle_change(res: NotifyResult<notify::Event>) -> () {
        match res {
            Ok(event) => {
                if let Some(path) = event.paths.first() {
                    tokio::spawn(async move {
                        if let Err(e) = Self::process_change(path).await {
                            error!("Error processing change: {}", e);
                        }
                    });
                }
            }
            Err(e) => error!("Watch error: {}", e),
        }
    }

    async fn process_change(path: &Path) -> Result<()> {
        // Get file metadata
        let metadata = fs::metadata(path).await?;
        
        // Process based on system layer
        match self.layer_map.get(path) {
            Some(SystemLayer::Layer1Core) => {
                // Verify no core principles are modified
                self.verify_core_principles(path).await?;
            },
            Some(SystemLayer::Layer2Extensions) => {
                // Process Layer 2 changes
                self.process_layer2_change(path).await?;
            },
            Some(SystemLayer::Layer3Applications) => {
                // Adapt ML system to changes
                self.adapt_ml_system(path).await?;
            },
            None => {
                // Categorize new paths
                let layer = self.detect_layer(path).await?;
                self.layer_map.insert(path.to_path_buf(), layer);
            }
        }

        Ok(())
    }

    async fn adapt_ml_system(&self, path: &Path) -> Result<()> {
        let mut ml_manager = self.ml_manager.lock().await;
        
        // Analyze changes
        let changes = self.analyze_file_changes(path).await?;
        
        // Update ML models based on changes
        ml_manager.adapt_to_changes(changes).await?;
        
        // Run test suite
        self.run_test_suite().await?;
        
        Ok(())
    }

    async fn run_test_suite(&self) -> Result<()> {
        info!("Running test suite for system changes");
        
        // Run core tests first
        self.run_core_tests().await?;
        
        // Run integration tests
        self.run_integration_tests().await?;
        
        // Verify system stability
        self.verify_system_stability().await?;
        
        Ok(())
    }
}

