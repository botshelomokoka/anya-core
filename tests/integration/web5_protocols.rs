use super::*;
use anya_core::auth::web5::protocols::{bitcoin, ml};

#[tokio::test]
async fn test_bitcoin_protocol() {
    let protocol = bitcoin::get_bitcoin_protocol();
    assert_eq!(protocol.protocol_id, bitcoin::BITCOIN_PROTOCOL_ID);
    
    // Test schema validation
    let test_tx = serde_json::json!({
        "txid": "abc123",
        "psbt": "cde456",
        "status": "pending",
        "metadata": {
            "fee_rate": 5.0
        }
    });
    
    let schema = &protocol.types[0].schema;
    let validation = jsonschema::validate(schema, &test_tx);
    assert!(validation.is_ok());
}

#[tokio::test]
async fn test_ml_protocol() {
    let protocol = ml::get_ml_protocol();
    assert_eq!(protocol.protocol_id, ml::ML_PROTOCOL_ID);
    
    // Test ML schema validation
    let test_training = serde_json::json!({
        "model_id": "model123",
        "training_data": [],
        "hyperparameters": {},
        "metrics": {}
    });
    
    let schema = &protocol.types[0].schema;
    let validation = jsonschema::validate(schema, &test_training);
    assert!(validation.is_ok());
}
