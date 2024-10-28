use bitcoin::OutPoint;
use bitcoin::TxOut;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UTXOManager {
    utxos: HashMap<OutPoint, UTXOEntry>,
    db: Database, // Your database connection
}

#[derive(Debug, Serialize, Deserialize)]
struct UTXOEntry {
    txout: TxOut,
    confirmations: u32,
    is_spent: bool,
}

impl UTXOManager {
    pub async fn add_utxo(&mut self, outpoint: OutPoint, txout: TxOut) -> Result<(), DBError> {
        let entry = UTXOEntry {
            txout,
            confirmations: 0,
            is_spent: false,
        };
        
        self.db.insert_utxo(&outpoint, &entry).await?;
        self.utxos.insert(outpoint, entry);
        Ok(())
    }

    pub async fn get_spendable_utxos(&self) -> Vec<(OutPoint, &TxOut)> {
        self.utxos.iter()
            .filter(|(_, entry)| !entry.is_spent && entry.confirmations >= 6)
            .map(|(outpoint, entry)| (*outpoint, &entry.txout))
            .collect()
    }
}
