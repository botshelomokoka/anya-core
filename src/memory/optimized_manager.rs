use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;
use log::{info, warn, error};
use metrics::{counter, gauge};

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Allocation failed: {0}")]
    AllocationError(String),
    #[error("Memory limit exceeded: {0}")]
    MemoryLimitError(String),
    #[error("Fragmentation error: {0}")]
    FragmentationError(String),
}

pub struct OptimizedMemoryManager {
    total_memory: usize,
    allocated: Arc<Mutex<usize>>,
    page_size: usize,
    metrics: MemoryMetrics,
    defrag_threshold: f64,
}

impl OptimizedMemoryManager {
    pub fn new(total_memory: usize) -> Self {
        Self {
            total_memory,
            allocated: Arc::new(Mutex::new(0)),
            page_size: 4096, // 4KB pages
            metrics: MemoryMetrics::new(),
            defrag_threshold: 0.7, // 70% fragmentation threshold
        }
    }

    pub async fn allocate(&self, size: usize) -> Result<MemoryAllocation, MemoryError> {
        let aligned_size = self.align_to_page(size);
        let mut allocated = self.allocated.lock().await;

        if *allocated + aligned_size > self.total_memory {
            // Try memory defragmentation if needed
            if self.should_defragment().await {
                self.defragment().await?;
            }

            // Check again after defragmentation
            if *allocated + aligned_size > self.total_memory {
                return Err(MemoryError::MemoryLimitError(
                    format!("Cannot allocate {} bytes, only {} available", 
                    aligned_size, self.total_memory - *allocated)
                ));
            }
        }

        *allocated += aligned_size;
        self.metrics.record_allocation(aligned_size);

        Ok(MemoryAllocation {
            size: aligned_size,
            pages: aligned_size / self.page_size,
        })
    }

    pub async fn deallocate(&self, allocation: MemoryAllocation) -> Result<(), MemoryError> {
        let mut allocated = self.allocated.lock().await;
        *allocated = allocated.saturating_sub(allocation.size);
        self.metrics.record_deallocation(allocation.size);
        Ok(())
    }

    async fn should_defragment(&self) -> bool {
        let fragmentation_ratio = self.calculate_fragmentation().await;
        fragmentation_ratio > self.defrag_threshold
    }

    async fn calculate_fragmentation(&self) -> f64 {
        let allocated = *self.allocated.lock().await;
        let used_pages = allocated / self.page_size;
        let total_pages = self.total_memory / self.page_size;
        
        1.0 - (used_pages as f64 / total_pages as f64)
    }

    async fn defragment(&self) -> Result<(), MemoryError> {
        info!("Starting memory defragmentation");
        
        // Implement memory defragmentation logic
        let mut allocated = self.allocated.lock().await;
        let fragmentation_before = self.calculate_fragmentation().await;
        
        // Compact memory pages
        // This is a simplified example - real implementation would move memory pages
        *allocated = self.compact_memory(*allocated);
        
        let fragmentation_after = self.calculate_fragmentation().await;
        self.metrics.record_defragmentation(fragmentation_before, fragmentation_after);
        
        info!("Memory defragmentation complete. Fragmentation reduced from {:.2}% to {:.2}%", 
            fragmentation_before * 100.0, 
            fragmentation_after * 100.0);
        
        Ok(())
    }

    fn align_to_page(&self, size: usize) -> usize {
        (size + self.page_size - 1) / self.page_size * self.page_size
    }

    fn compact_memory(&self, allocated: usize) -> usize {
        // Implement memory compaction logic
        // This is a simplified version - real implementation would reorganize memory pages
        allocated
    }
}

pub struct MemoryAllocation {
    size: usize,
    pages: usize,
}

struct MemoryMetrics {
    total_allocated: Gauge,
    allocation_count: Counter,
    deallocation_count: Counter,
    fragmentation_ratio: Gauge,
    defragmentation_count: Counter,
}

impl MemoryMetrics {
    fn new() -> Self {
        Self {
            total_allocated: gauge!("memory_total_allocated_bytes"),
            allocation_count: counter!("memory_allocations_total"),
            deallocation_count: counter!("memory_deallocations_total"),
            fragmentation_ratio: gauge!("memory_fragmentation_ratio"),
            defragmentation_count: counter!("memory_defragmentations_total"),
        }
    }

    fn record_allocation(&self, size: usize) {
        self.total_allocated.add(size as f64);
        self.allocation_count.increment(1);
    }

    fn record_deallocation(&self, size: usize) {
        self.total_allocated.sub(size as f64);
        self.deallocation_count.increment(1);
    }

    fn record_defragmentation(&self, before: f64, after: f64) {
        self.fragmentation_ratio.set(after);
        self.defragmentation_count.increment(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_allocation() {
        let manager = OptimizedMemoryManager::new(1024 * 1024); // 1MB
        let allocation = manager.allocate(4096).await.unwrap();
        assert_eq!(allocation.size, 4096);
        assert_eq!(allocation.pages, 1);
    }

    #[tokio::test]
    async fn test_memory_deallocation() {
        let manager = OptimizedMemoryManager::new(1024 * 1024);
        let allocation = manager.allocate(4096).await.unwrap();
        assert!(manager.deallocate(allocation).await.is_ok());
    }

    #[tokio::test]
    async fn test_memory_fragmentation() {
        let manager = OptimizedMemoryManager::new(1024 * 1024);
        let fragmentation = manager.calculate_fragmentation().await;
        assert!(fragmentation >= 0.0 && fragmentation <= 1.0);
    }
}
