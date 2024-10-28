use super::*;
use anya_core::auth::web5::{Web5DataManager, DataRecord};
use did_key::Ed25519KeyPair;

#[tokio::test]
async fn test_web5_data_storage() {
    let key_pair = Ed25519KeyPair::generate();
    let data_manager = Web5DataManager::new(key_pair).await.unwrap();
    
    let test_record = DataRecord {
        protocol_id: "test".to_string(),
        schema: "TestSchema".to_string(),
        data: serde_json::json!({
            "test": "data"
        }),
        timestamp: chrono::Utc::now(),
    };
    
    let record_id = data_manager.store_data(test_record).await.unwrap();
    assert!(!record_id.is_empty());
}

#[tokio::test]
async fn test_web5_sync() {
    let key_pair = Ed25519KeyPair::generate();
    let data_manager = Web5DataManager::new(key_pair).await.unwrap();
    let sync_manager = DwnSyncManager::new(Duration::from_secs(60));
    
    sync_manager.sync_records(&data_manager).await.unwrap();
}
