use std::error::Error;

use bitcoin::{
    Network as BitcoinNetwork,
    Address as BitcoinAddress,
    util::key::PrivateKey,
    util::psbt::PartiallySignedTransaction,
    util::key::PrivateKey,
    util::psbt::PartiallySignedTransaction,
};

use bitcoincore_rpc::{Auth, Client, RpcApi};
use secp256k1::Secp256k1;
use log::{info, error, warn};
use tokio::time::Duration;

pub struct BitcoinSupport {
    network: BitcoinNetwork,
    client: Client,
    secp: Secp256k1<secp256k1::All>,
}

impl BitcoinSupport {
    pub fn new(network: BitcoinNetwork, rpc_url: &str, rpc_user: &str, rpc_password: &str) -> Result<Self, Box<dyn Error>> {
        let auth = Auth::UserPass(rpc_user.to_owned(), rpc_password.to_owned());
        let client = Client::new(rpc_url, auth)?;
        let secp = Secp256k1::new();

        Ok(Self {
            network,
            client,
            secp,
            client,
            secp,
        })
    }
}

impl BitcoinSupport {
    pub fn get_balance(&self, address: &BitcoinAddress) -> Result<u64, Box<dyn Error>> {
        let balance = self.client.get_received_by_address(address, None)?;
        Ok(balance)
    }

    pub fn create_and_sign_transaction(&self, from_address: &BitcoinAddress, to_address: &BitcoinAddress, amount: u64, private_key: &PrivateKey) -> Result<PartiallySignedTransaction, Box<dyn Error>> {
        let utxos = self.list_unspent_utxos(from_address)?;
        let (mut tx_builder, total_input) = self.create_transaction_builder(&utxos, amount)?;
        self.add_outputs(&mut tx_builder, from_address, to_address, amount, total_input)?;
        let psbt = self.sign_transaction(tx_builder, private_key)?;
        Ok(psbt)
    }

    fn list_unspent_utxos(&self, address: &BitcoinAddress) -> Result<Vec<bitcoin::util::psbt::Input>, Box<dyn Error>> {
        let utxos = self.client.list_unspent(None, None, Some(&[address]), None, None)?;
        Ok(utxos)
    }

    fn create_transaction_builder(&self, utxos: &[bitcoin::util::psbt::Input], amount: u64) -> Result<(bitcoin::util::psbt::PartiallySignedTransaction, u64), Box<dyn Error>> {
        let mut tx_builder = bitcoin::util::psbt::PartiallySignedTransaction::from_unsigned_tx(bitcoin::Transaction {
            version: 2,
            lock_time: 0,
            input: vec![],
            output: vec![],
        });

        let mut total_input = 0;
        for utxo in utxos {
            if total_input >= amount {
            if total_input >= amount {
                break;
            }
            tx_builder.input.push(bitcoin::util::psbt::Input {
                non_witness_utxo: Some(utxo.tx_out().clone()),
                ..Default::default()
            });
            total_input += utxo.amount.to_sat();
        }

        if total_input < amount {
            return Err("Insufficient funds to create the transaction".into());
        }

        Ok((tx_builder, total_input))
    }

    fn add_outputs(&self, tx_builder: &mut bitcoin::util::psbt::PartiallySignedTransaction, from_address: &BitcoinAddress, to_address: &BitcoinAddress, amount: u64, total_input: u64) -> Result<(), Box<dyn Error>> {
        tx_builder.unsigned_tx.output.push(bitcoin::TxOut {
            amount: amount,
            script_pubkey: to_address.script_pubkey(),
            ..Default::default()
        });
            ..Default::default()
        });

        let change = total_input - amount;
        if change > 0 {
            tx_builder.unsigned_tx.output.push(bitcoin::TxOut {
                amount: change,
                script_pubkey: from_address.script_pubkey(),
                ..Default::default()
                ..Default::default()
            });
        }

        Ok(())
    }

    fn sign_transaction(&self, tx_builder: bitcoin::util::psbt::PartiallySignedTransaction, private_key: &PrivateKey) -> Result<PartiallySignedTransaction, Box<dyn Error>> {
        let mut psbt = bitcoin::util::psbt::PartiallySignedTransaction::from(tx_builder);
        let secp = Secp256k1::new();
        psbt.sign(&private_key, &secp)?;
        Ok(psbt)
    }

    pub fn broadcast_transaction(&self, psbt: &PartiallySignedTransaction) -> Result<String, Box<dyn Error>> {
        let tx = psbt.extract_tx();
        let txid = self.client.send_raw_transaction(&tx)?;
        Ok(txid.to_string())
    }

    pub fn get_network_info(&self) -> Result<bitcoincore_rpc::json::GetNetworkInfoResult, Box<dyn Error>> {
        let network_info = self.client.get_network_info()?;
        Ok(network_info)
    }

    pub async fn get_network_performance(&self) -> Result<f64, Box<dyn Error>> {
        let transaction_throughput = self.get_transaction_throughput().await?;
        let block_time = self.get_average_block_time().await?;
        let fee_rate = self.get_average_fee_rate().await?;
        
        // Combine metrics into a single performance score
        Ok(0.4 * transaction_throughput + 0.3 * (1.0 / block_time) + 0.3 * (1.0 / fee_rate))
    }

    async fn get_average_block_time(&self) -> Result<f64, Box<dyn Error>> {
        // Implement logic to get average block time
        Ok(600.0) // Seconds, placeholder value
    }

    async fn get_average_fee_rate(&self) -> Result<f64, Box<dyn Error>> {
    pub async fn handle_bitcoin_operations(&self, shutdown: tokio::sync::watch::Receiver<()>) {
        let mut sleep_duration = Duration::from_secs(300);

        tokio::select! {
            _ = shutdown.changed() => {
                self.handle_shutdown().await;
            }
            _ = tokio::time::sleep(sleep_duration) => {
                self.perform_bitcoin_operations(&mut sleep_duration).await;
            }
        }
    }

    async fn handle_shutdown(&self) {
        info!("Shutdown signal received, stopping bitcoin operations.");
    }

    async fn perform_bitcoin_operations(&self, sleep_duration: &mut Duration) {
        match self.get_network_performance().await {
            Ok(performance) => {
                info!("Bitcoin network performance: {}", performance);
                // Adjust sleep duration based on performance
                *sleep_duration = Duration::from_secs((300.0 / performance) as u64);
            }
            Err(e) => warn!("Failed to get Bitcoin network performance: {:?}", e),
        }

        match self.get_balance_async().await {
            Ok(balance) => info!("Current Bitcoin balance: {} BTC", balance),
            Err(e) => warn!("Failed to get Bitcoin balance: {:?}", e),
        }

        // Add more Bitcoin-related operations here
    }
                match self.get_balance_async().await {
                    Ok(balance) => info!("Current Bitcoin balance: {} BTC", balance),
                    Err(e) => warn!("Failed to get Bitcoin balance: {:?}", e),
                }

                // Add more Bitcoin-related operations here
            }
        }
    }

    // Add more Bitcoin-related operations as needed
}                       Ok(balance) => info!("Current Bitcoin balance: {} BTC", balance),
                        Err(e) => warn!("Failed to get Bitcoin balance: {:?}", e),
                    }

                    // Add more Bitcoin-related operations here
                }
            }
        }
    }

    // Add more Bitcoin-related operations as needed
}               }
            }
        }
    }

    // Add more Bitcoin-related operations as needed
}
