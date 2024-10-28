use super::super::data_manager::{ProtocolDefinition, SchemaDefinition};
use serde_json::json;

pub const BITCOIN_PROTOCOL_ID: &str = "https://anya.protocol/bitcoin/v1";

pub fn get_bitcoin_protocol() -> ProtocolDefinition {
    ProtocolDefinition {
        protocol_id: BITCOIN_PROTOCOL_ID.to_string(),
        types: vec![
            SchemaDefinition {
                schema_id: "Transaction".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "txid": { "type": "string" },
                        "psbt": { "type": "string" },
                        "status": { "type": "string" },
                        "metadata": { "type": "object" }
                    }
                }),
            },
            SchemaDefinition {
                schema_id: "UTXO".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "outpoint": { "type": "string" },
                        "amount": { "type": "number" },
                        "script": { "type": "string" },
                        "confirmations": { "type": "number" }
                    }
                }),
            },
        ],
        rules: vec![
            // Add Bitcoin-specific rules
        ],
    }
}
