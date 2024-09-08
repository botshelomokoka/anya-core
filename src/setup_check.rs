use log::{info, warn, error};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use crate::user_management::UserType;
use crate::setup_project::ProjectSetup;
use crate::zk_utils::ZKSetup;
use crate::stx_support::STXSupport;
use crate::dlc_support::DLCSupport;
use crate::lightning_support::LightningSupport;
use crate::bitcoin_support::BitcoinSupport;
use crate::web5_support::Web5Support;
use stacks_core::{
    StacksAddress, StacksPublicKey, StacksPrivateKey, StacksTransaction, StacksNetwork, StacksEpochId,
    clarity::types::QualifiedContractIdentifier,
    chainstate::stacks::StacksBlockId,
    chainstate::burn::ConsensusHash,
};
use stacks_rpc_client::{
    StacksRpcClient, PoxInfo, AccountBalanceResponse, TransactionStatus,
    BlockInfoResponse, BurnchainRewardSlotHolderResponse,
};
use bitcoin::{
    Network as BitcoinNetwork, Address as BitcoinAddress, Transaction, TxIn, TxOut, OutPoint,
    blockdata::script::Script, util::amount::Amount,
};
use lightning::{
    chain::keysinterface::KeysManager,
    ln::{channelmanager::ChannelManager, chan_utils::ChannelId},
    util::config::UserConfig,
};
use dlc::{DlcManager, OracleInfo, Offer, Contract, Outcome};
use libp2p::{
    identity, PeerId, Swarm,
    core::upgrade,
    futures::StreamExt,
    noise, mplex, yamux,
    swarm::SwarmBuilder,
    tcp::TokioTcpConfig,
    kad::Kademlia,
    gossipsub::{Gossipsub, GossipsubConfig},
};
use web5::{
    did::{DID, KeyMethod},
    dids::methods::key::DIDKey,
    credentials::{Credential, CredentialSubject, CredentialStatus},
};

fn check_requirements() -> Result<Vec<String>, io::Error> {
    let requirements_path = "requirements.txt";
    let path = Path::new(requirements_path);
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Requirements file not found: {}", requirements_path)));
    }

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut missing = Vec::new();
    for line in reader.lines() {
        let requirement = line?.trim().to_string();
        if !requirement.is_empty() && !requirement.starts_with('#') {
            missing.push(requirement);
        }
    }

    Ok(missing)
}

pub async fn check_and_fix_setup(user_type: UserType, user_data: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Checking setup for user type: {:?}", user_type);

    let missing_packages = check_requirements()?;
    if !missing_packages.is_empty() {
        warn!("Missing packages: {}. Please install them.", missing_packages.join(", "));
        return Ok(());
    }

    let mut project_setup = ProjectSetup::new(user_type, user_data.clone());

    if !project_setup.check_common_environment() {
        warn!("Common environment setup incomplete. Fixing...");
        project_setup.setup_common_environment()?;
    }

    match user_type {
        UserType::Creator => {
            if !project_setup.check_creator_setup() {
                warn!("Creator-specific setup incomplete. Fixing...");
                project_setup.setup_creator_project()?;
            }
        },
        UserType::Developer => {
            if !project_setup.check_developer_setup() {
                warn!("Developer-specific setup incomplete. Fixing...");
                project_setup.setup_developer_project()?;
            }
        },
        UserType::Normal => {
            if !project_setup.check_normal_user_setup() {
                warn!("Normal user-specific setup incomplete. Fixing...");
                project_setup.setup_normal_user_project()?;
            }
        },
    }

    let mut zk_setup = ZKSetup::new(user_type, user_data.clone());
    if !zk_setup.check_zk_environment() {
        warn!("ZK environment setup incomplete. Fixing...");
        zk_setup.setup_zk_environment()?;
    }

    // STX Support
    let mut stx_support = STXSupport::new(user_type, user_data.clone());
    if !stx_support.check_stx_environment() {
        warn!("STX environment setup incomplete. Fixing...");
        stx_support.setup_stx_environment()?;
    }

    let stacks_rpc_client = StacksRpcClient::new("https://stacks-node-api.mainnet.stacks.co");
    let stx_address = StacksAddress::from_string(&user_data["stx_address"])?;
    let balance_response: AccountBalanceResponse = stacks_rpc_client.get_account_balance(&stx_address).await?;
    info!("STX balance: {}", balance_response.stx.balance);

    let pox_info: PoxInfo = stacks_rpc_client.get_pox_info().await?;
    info!("Current PoX cycle: {}", pox_info.current_cycle.id);

    if user_type == UserType::Developer {
        let contract_name = "my-contract";
        let contract_source = include_str!("../contracts/my-contract.clar");
        let contract_id = QualifiedContractIdentifier::new(stx_address.clone(), contract_name.to_string());
        let tx_status = stx_support.deploy_contract(&contract_id, contract_source).await?;
        info!("Contract deployment status: {:?}", tx_status);

        let block_info: BlockInfoResponse = stacks_rpc_client.get_block_by_height(pox_info.current_cycle.id).await?;
        info!("Latest block info: {:?}", block_info);

        let reward_slots: Vec<BurnchainRewardSlotHolderResponse> = stacks_rpc_client.get_burnchain_reward_slot_holders(pox_info.current_cycle.id).await?;
        info!("Current reward slot holders: {:?}", reward_slots);
    }

    // DLC Support
    let mut dlc_support = DLCSupport::new(user_type, user_data.clone());
    if !dlc_support.check_dlc_environment() {
        warn!("DLC environment setup incomplete. Fixing...");
        dlc_support.setup_dlc_environment()?;
    }

    let dlc_manager = DlcManager::new()?;
    let oracle_info = OracleInfo::new("example_oracle", "https://example.com/oracle");
    dlc_support.register_oracle(oracle_info)?;
    info!("DLC environment set up successfully");

    let collateral = 1_000_000; // in satoshis
    let oracle_event = "btc_price_2023_12_31";
    let offer = dlc_support.create_dlc_offer(collateral, oracle_event)?;
    info!("Created DLC offer: {:?}", offer);

    let contract = dlc_support.accept_dlc_offer(&offer)?;
    info!("DLC contract created: {:?}", contract);

    // Lightning Support
    let mut lightning_support = LightningSupport::new(user_type, user_data.clone());
    if !lightning_support.check_lightning_environment() {
        warn!("Lightning environment setup incomplete. Fixing...");
        lightning_support.setup_lightning_environment()?;
    }

    let keys_manager = KeysManager::new(&[0u8; 32], 42, 42);
    let user_config = UserConfig::default();
    let channel_manager = ChannelManager::new(
        lightning_support.get_fee_estimator(),
        &lightning_support.get_chain_monitor(),
        &lightning_support.get_tx_broadcaster(),
        &lightning_support.get_logger(),
        &keys_manager,
        &user_config,
        lightning_support.get_current_blockchain_tip().await?,
    )?;
    info!("Lightning environment set up successfully");

    let node_pubkey = "027abc..."; // Example node public key
    let channel_value_satoshis = 1_000_000;
    let push_msat = 0;
    let channel_open_result = lightning_support.open_channel(&channel_manager, node_pubkey, channel_value_satoshis, push_msat).await?;
    info!("Lightning channel opened: {:?}", channel_open_result);

    // Bitcoin Support
    let mut bitcoin_support = BitcoinSupport::new(user_type, user_data.clone());
    if !bitcoin_support.check_bitcoin_environment() {
        warn!("Bitcoin environment setup incomplete. Fixing...");
        bitcoin_support.setup_bitcoin_environment()?;
    }

    let bitcoin_network = BitcoinNetwork::Bitcoin;
    let bitcoin_address = BitcoinAddress::from_str(&user_data["bitcoin_address"])?;
    let bitcoin_balance = bitcoin_support.get_balance(&bitcoin_address).await?;
    info!("Bitcoin balance: {} satoshis", bitcoin_balance);

    let recipient_address = BitcoinAddress::from_str("1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2")?;
    let amount_satoshis = 50_000;
    let tx_id = bitcoin_support.send_bitcoin(&bitcoin_address, &recipient_address, amount_satoshis).await?;
    info!("Bitcoin transaction sent. Transaction ID: {}", tx_id);

    // Web5 Support
    let mut web5_support = Web5Support::new(user_type, user_data.clone());
    if !web5_support.check_web5_environment() {
        warn!("Web5 environment setup incomplete. Fixing...");
        web5_support.setup_web5_environment()?;
    }

    let did_key = DIDKey::generate(KeyMethod::Ed25519)?;
    info!("Generated DID: {}", did_key.to_did());

    let credential = Credential::new(
        "ExampleCredential",
        vec!["VerifiableCredential", "ExampleCredential"],
        did_key.to_did(),
        CredentialSubject::new(HashMap::from([
            ("id".to_string(), did_key.to_did()),
            ("name".to_string(), "Alice".to_string()),
        ])),
        None,
    );
    info!("Created credential: {:?}", credential);

    // Libp2p Support
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    info!("Local peer id: {:?}", peer_id);

    let transport = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(id_keys).into_authenticated())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    let mut swarm = {
        let kademlia = Kademlia::new(peer_id.clone(), libp2p::kad::store::MemoryStore::new(peer_id.clone()));
        let gossipsub_config = GossipsubConfig::default();
        let gossipsub = Gossipsub::new(MessageAuthenticity::Signed(id_keys.clone()), gossipsub_config).unwrap();
        
        SwarmBuilder::new(transport, libp2p::behaviour::Behaviour::new(kademlia, gossipsub), peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build()
    };

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let remote_peer_id = "QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ";
    let remote_addr = "/ip4/104.131.131.82/tcp/4001".parse()?;
    swarm.dial(remote_addr)?;
    info!("Dialed remote peer: {}", remote_peer_id);

    info!("Setup check and fix completed successfully");
    Ok(())
}
