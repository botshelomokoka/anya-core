use std::env;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use reqwest;
use serde_json::Value;
use log::{info, error};
use crypto::aes::{cbc_encryptor, cbc_decryptor, KeySize};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer, BufferResult};
use rand::Rng;
use crate::setup_project::ProjectSetup;
use crate::stx_support::STXSupport;
use crate::dlc_support::DLCSupport;
use crate::lightning_support::LightningSupport;
use crate::bitcoin_support::BitcoinSupport;
use crate::web5_support::Web5Support;
use crate::libp2p_support::Libp2pSupport;

// Stacks imports
use stacks_common::types::StacksAddress;
use stacks_common::types::StacksPublicKey;
use stacks_common::types::StacksPrivateKey;
use stacks_transactions::StacksTransaction;
use stacks_common::types::StacksNetwork;
use stacks_common::types::StacksEpochId;
use clarity_repl::clarity::types::QualifiedContractIdentifier;
use stacks_rpc_client::StacksRpcClient;
use stacks_rpc_client::PoxInfo;
use stacks_rpc_client::AccountBalanceResponse;
use stacks_rpc_client::TransactionStatus;

// Bitcoin and Lightning imports
use bitcoin::Network as BitcoinNetwork;
use bitcoin::Address as BitcoinAddress;
use bitcoin::PublicKey as BitcoinPublicKey;
use bitcoin::PrivateKey as BitcoinPrivateKey;
use lightning::chain::keysinterface::KeysManager;
use lightning::ln::channelmanager::ChannelManager;
use lightning::util::events::Event;

// DLC imports
use dlc::DlcManager;
use dlc::OracleInfo;
use dlc::Contract as DlcContract;

// Libp2p imports
use libp2p::PeerId;
use libp2p::identity;
use libp2p::Swarm;
use libp2p::NetworkBehaviour;

// Web5 imports
use web5::did::{DID, DIDDocument};
use web5::credentials::{Credential, VerifiableCredential};

#[derive(Default, Debug)]
struct UserState {
    github_username:    String,
    user_type:          String,
    encrypted_data:     HashMap<String, Vec<u8>>,
    stx_address:        Option<StacksAddress>,
    stx_public_key:     Option<StacksPublicKey>,
    stx_private_key:    Option<StacksPrivateKey>,
    bitcoin_address:    Option<BitcoinAddress>,
    bitcoin_public_key: Option<BitcoinPublicKey>,
    bitcoin_private_key:Option<BitcoinPrivateKey>,
    lightning_node_id:  Option<String>,
    lightning_channels: Vec<ChannelManager>,
    dlc_pubkey:         Option<String>,
    dlc_contracts:      Vec<DlcContract>,
    web5_did:           Option<DID>,
    web5_credentials:   Vec<VerifiableCredential>,
    libp2p_peer_id:     Option<PeerId>,
}

struct UserType;

impl UserType {
    const CREATOR:   &'static str = "creator";
    const NORMAL:    &'static str = "normal";
    const DEVELOPER: &'static str = "developer";
}

pub struct UserManagement {
    logger:            log::Logger,
    github_token:      Option<String>,
    user_state:        UserState,
    cipher_key:        [u8; 32],
    stx_support:       STXSupport,
    dlc_support:       DLCSupport,
    lightning_support: LightningSupport,
    bitcoin_support:   BitcoinSupport,
    web5_support:      Web5Support,
    libp2p_support:    Libp2pSupport,
}

impl UserManagement {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let cipher_key: [u8; 32] = rng.gen();
        
        Ok(UserManagement {
            logger: log::Logger::root(log::slog_stdlog::StdLog.fuse(), o!()),
            github_token: env::var("GITHUB_TOKEN").ok(),
            user_state: UserState::default(),
            cipher_key,
            stx_support: STXSupport::new()?,
            dlc_support: DLCSupport::new()?,
            lightning_support: LightningSupport::new()?,
            bitcoin_support: BitcoinSupport::new()?,
            web5_support: Web5Support::new()?,
            libp2p_support: Libp2pSupport::new()?,
        })
    }

    pub async fn identify_user(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(github_username) = self.get_github_username().await? {
            self.user_state.github_username = github_username.clone();
            if github_username == "botshelomokoka" {
                self.user_state.user_type = UserType::CREATOR.to_string();
                info!(self.logger, "Creator identified. Setting up creator-specific configurations.");
            } else if self.is_developer(&github_username).await? {
                self.user_state.user_type = UserType::DEVELOPER.to_string();
                info!(self.logger, "Developer identified. Setting up developer environment.");
            } else {
                self.user_state.user_type = UserType::NORMAL.to_string();
                info!(self.logger, "Normal user identified.");
            }
        } else {
            error!(self.logger, "Failed to identify user.");
        }
        Ok(())
    }

    async fn get_github_username(&self) -> Result<Option<String>, Box<dyn Error>> {
        match &self.github_token {
            Some(token) => {
                let client = reqwest::Client::new();
                let response = client.get("https://api.github.com/user")
                    .header("Authorization", format!("token {}", token))
                    .header("Accept", "application/vnd.github.v3+json")
                    .send()
                    .await?
                    .json::<Value>()
                    .await?;
                Ok(response["login"].as_str().map(|s| s.to_string()))
            }
            None => {
                error!(self.logger, "GitHub token not found in environment variables.");
                Ok(None)
            }
        }
    }

    async fn is_developer(&self, github_username: &str) -> Result<bool, Box<dyn Error>> {
        let developer_organizations = vec!["anya-core-developers"];
        let developer_teams = vec!["dev-team"];

        if let Some(token) = &self.github_token {
            let client = reqwest::Client::new();
            for org in developer_organizations {
                let response = client.get(&format!("https://api.github.com/orgs/{}/members/{}", org, github_username))
                    .header("Authorization", format!("token {}", token))
                    .header("Accept", "application/vnd.github.v3+json")
                    .send()
                    .await?;
                if response.status() == 204 {
                    return Ok(true);
                }

                for team in &developer_teams {
                    let response = client.get(&format!("https://api.github.com/orgs/{}/teams/{}/memberships/{}", org, team, github_username))
                        .header("Authorization", format!("token {}", token))
                        .header("Accept", "application/vnd.github.v3+json")
                        .send()
                        .await?;
                    if response.status() == 200 {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }

    pub fn encrypt_user_data(&mut self, data: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        for (key, value) in data {
            let encrypted_value = self.encrypt(&value)?;
            self.user_state.encrypted_data.insert(key, encrypted_value);
        }
        Ok(())
    }

    pub fn decrypt_user_data(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        if let Some(encrypted_value) = self.user_state.encrypted_data.get(key) {
            Ok(Some(self.decrypt(encrypted_value)?))
        } else {
            Ok(None)
        }
    }

    fn encrypt(&self, data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut encryptor = cbc_encryptor(
            KeySize::KeySize256,
            &self.cipher_key,
            &[0u8; 16],
            crypto::blockmodes::PkcsPadding,
        );

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = RefReadBuffer::new(data.as_bytes());
        let mut buffer = [0; 4096];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);

        loop {
            let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }

        Ok(final_result)
    }

    fn decrypt(&self, encrypted_data: &[u8]) -> Result<String, Box<dyn Error>> {
        let mut decryptor = cbc_decryptor(
            KeySize::KeySize256,
            &self.cipher_key,
            &[0u8; 16],
            crypto::blockmodes::PkcsPadding,
        );

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = RefReadBuffer::new(encrypted_data);
        let mut buffer = [0; 4096];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);

        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }

        Ok(String::from_utf8(final_result)?)
    }

    pub fn get_user_state(&self) -> HashMap<String, String> {
        let mut state = HashMap::new();
        state.insert("github_username".to_string(), self.user_state.github_username.clone());
        state.insert("user_type".to_string(), self.user_state.user_type.clone());
        if let Some(stx_address) = &self.user_state.stx_address {
            state.insert("stx_address".to_string(), stx_address.to_string());
        }
        if let Some(bitcoin_address) = &self.user_state.bitcoin_address {
            state.insert("bitcoin_address".to_string(), bitcoin_address.to_string());
        }
        if let Some(lightning_node_id) = &self.user_state.lightning_node_id {
            state.insert("lightning_node_id".to_string(), lightning_node_id.clone());
        }
        if let Some(dlc_pubkey) = &self.user_state.dlc_pubkey {
            state.insert("dlc_pubkey".to_string(), dlc_pubkey.clone());
        }
        if let Some(web5_did) = &self.user_state.web5_did {
            state.insert("web5_did".to_string(), web5_did.to_string());
        }
        if let Some(libp2p_peer_id) = &self.user_state.libp2p_peer_id {
            state.insert("libp2p_peer_id".to_string(), libp2p_peer_id.to_string());
        }
        state
    }

    pub async fn initialize_user(&mut self) -> Result<(), Box<dyn Error>> {
        self.identify_user().await?;
        match self.user_state.user_type.as_str() {
            UserType::CREATOR => self.setup_creator_environment().await?,
            UserType::DEVELOPER => self.setup_developer_environment().await?,
            _ => self.setup_normal_user_environment().await?,
        }
        self.setup_project()?;
        Ok(())
    }

    async fn setup_creator_environment(&mut self) -> Result<(), Box<dyn Error>> {
        info!(self.logger, "Setting up creator environment");
        self.setup_stx_environment().await?;
        self.setup_bitcoin_environment().await?;
        self.setup_lightning_environment().await?;
        self.setup_dlc_environment().await?;
        self.setup_web5_environment().await?;
        self.setup_libp2p_environment().await?;
        Ok(())
    }

    async fn setup_developer_environment(&mut self) -> Result<(), Box<dyn Error>> {
        info!(self.logger, "Setting up developer environment");
        self.setup_stx_environment().await?;
        self.setup_bitcoin_environment().await?;
        self.setup_lightning_environment().await?;
        self.setup_dlc_environment().await?;
        self.setup_web5_environment().await?;
        self.setup_libp2p_environment().await?;
        Ok(())
    }

    async fn setup_normal_user_environment(&mut self) -> Result<(), Box<dyn Error>> {
        info!(self.logger, "Setting up normal user environment");
        self.setup_stx_environment().await?;
        self.setup_bitcoin_environment().await?;
        self.setup_lightning_environment().await?;
        self.setup_dlc_environment().await?;
        self.setup_web5_environment().await?;
        self.setup_libp2p_environment().await?;
        Ok(())
    }

    async fn setup_stx_environment(&mut self) -> Result<(), Box<dyn Error>> {
        let (stx_address, stx_public_key, stx_private_key) = self.stx_support.generate_keys().await?;
        self.user_state.stx_address = Some(stx_address.clone());
        self.user_state.stx_public_key = Some(stx_public_key);
        self.user_state.stx_private_key = Some(stx_private_key);
        
        // Initialize STX wallet
        self.stx_support.initialize_wallet(&stx_address).await?;
        
        // Get STX balance
        let stx_balance = self.stx_support.get_balance(&stx_address).await?;
        info!(self.logger, "STX balance: {}", stx_balance);
        
        // Perform a sample STX transaction
        let recipient = StacksAddress::from_string("ST2CY5V39NHDPWSXMW9QDT3HC3GD6Q6XX4CFRK9AG")?;
        let amount = 100; // in microSTX
        let memo = "Test transaction".to_string();
        let tx_id = self.stx_support.send_transaction(&stx_address, &recipient, amount, &memo).await?;
        info!(self.logger, "STX transaction sent. Transaction ID: {}", tx_id);
        
        Ok(())
    }

    async fn setup_bitcoin_environment(&mut self) -> Result<(), Box<dyn Error>> {
        let (bitcoin_address, bitcoin_public_key, bitcoin_private_key) = self.bitcoin_support.generate_keys().await?;
        self.user_state.bitcoin_address = Some(bitcoin_address.clone());
        self.user_state.bitcoin_public_key = Some(bitcoin_public_key);
        self.user_state.bitcoin_private_key = Some(bitcoin_private_key);
        
        // Initialize Bitcoin wallet
        self.bitcoin_support.initialize_wallet(&bitcoin_address).await?;
        
        // Get Bitcoin balance
        let btc_balance = self.bitcoin_support.get_balance(&bitcoin_address).await?;
        info!(self.logger, "BTC balance: {}", btc_balance);
        
        // Perform a sample Bitcoin transaction
        let recipient = BitcoinAddress::from_str("1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2")?;
        let amount = 10000; // in satoshis
        let tx_id = self.bitcoin_support.send_transaction(&bitcoin_address, &recipient, amount).await?;
        info!(self.logger, "Bitcoin transaction sent. Transaction ID: {}", tx_id);
        
        Ok(())
    }

    async fn setup_lightning_environment(&mut self) -> Result<(), Box<dyn Error>> {
        let lightning_node_id = self.lightning_support.initialize_node().await?;
        self.user_state.lightning_node_id = Some(lightning_node_id.clone());
        
        // Open a sample channel
        let channel_amount = 1_000_000; // in satoshis
        let channel = self.lightning_support.open_channel(&lightning_node_id, channel_amount).await?;
        self.user_state.lightning_channels.push(channel);
        
        info!(self.logger, "Lightning node initialized with ID: {}", lightning_node_id);
        
        // Perform a sample Lightning payment
        let payment_hash = "0001020304050607080900010203040506070809000102030405060708090102";
        let amount_msat = 1000; // 1 satoshi
        Ok(())
    }

    async fn setup_dlc_environment(&mut self) -> Result<(), Box<dyn Error>> {
        let (dlc_pubkey, dlc_privkey) = self.dlc_support.generate_keypair().await?;
        self.user_state.dlc_pubkey = Some(dlc_pubkey.clone());
        
        // Create a sample DLC contract
        let oracle = OracleInfo::new("sample_oracle", "https://example.com/oracle");
        let contract = self.dlc_support.create_contract(&dlc_pubkey, &oracle, 1_000_000).await?;
        self.user_state.dlc_contracts.push(contract);
        
        info!(self.logger, "DLC environment set up with public key: {}", dlc_pubkey);
        
        Ok(())
    }

    fn setup_project(&self) -> Result<(), Box<dyn Error>> {
        let project_setup = ProjectSetup::new(&self.user_state.user_type, &self.get_user_state())?;
        project_setup.setup()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_management() -> Result<(), Box<dyn Error>> {
        let mut user_management = UserManagement::new()?;
        
        // Test user identification
        user_management.identify_user().await?;
        assert!(!user_management.user_state.github_username.is_empty());
        
        // Test encryption and decryption
        let mut test_data = HashMap::new();
        test_data.insert("test_key".to_string(), "test_value".to_string());
        user_management.encrypt_user_data(test_data)?;
        let decrypted_value = user_management.decrypt_user_data("test_key")?;
        assert_eq!(decrypted_value, Some("test_value".to_string()));
        
        // Test user initialization
        user_management.initialize_user().await?;
        let user_state = user_management.get_user_state();
        assert!(user_state.contains_key("stx_address"));
        assert!(user_state.contains_key("bitcoin_address"));
        
        Ok(())
    }
}
