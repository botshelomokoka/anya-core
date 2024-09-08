use std::sync::Arc;
use std::error::Error;
use bitcoin::network::constants::Network as BitcoinNetwork;
use lightning::{
    chain::keysinterface::KeysManager,
    ln::{
        channelmanager::{ChannelManager, ChannelManagerReadArgs},
        peer_handler::{MessageHandler, PeerManager},
        msgs::{ChannelMessageHandler, RoutingMessageHandler},
    },
    util::{
        config::UserConfig,
        events::Event,
        logger::Logger,
    },
    routing::router::{Route, RouteHop},
};
use lightning_invoice::Invoice;
use tokio;
use log::{info, error};

use crate::bitcoin_support::BitcoinSupport;

pub struct LightningSupport {
    network: BitcoinNetwork,
    keys_manager: Arc<KeysManager>,
    channel_manager: Arc<ChannelManager>,
    peer_manager: Arc<PeerManager>,
    bitcoin_support: Arc<BitcoinSupport>,
}

impl LightningSupport {
    pub async fn new(
        network: BitcoinNetwork,
        bitcoin_support: Arc<BitcoinSupport>,
    ) -> Result<Self, Box<dyn Error>> {
        let seed = [0u8; 32]; // This should be securely generated and stored
        let keys_manager = Arc::new(KeysManager::new(&seed, 0, 0));

        let logger = Arc::new(Logger::new());
        let user_config = UserConfig::default();

        let (channel_manager, _) = {
            let chain_monitor = Arc::new(ChainMonitor::new(None, &filter, &logger));
            let broadcaster = bitcoin_support.get_broadcaster();
            let fee_estimator = bitcoin_support.get_fee_estimator();
            let persister = YourPersisterImplementation::new();

            let channel_manager = ChannelManager::new(
                fee_estimator,
                chain_monitor.clone(),
                broadcaster,
                &logger,
                &keys_manager,
                user_config,
                &network,
            );

            let read_args = ChannelManagerReadArgs::new(
                keys_manager.clone(),
                fee_estimator,
                chain_monitor,
                broadcaster,
                &logger,
                user_config,
                &network,
            );

            match <(ChannelManager, Option<ChannelMonitor>)>::read(&mut persister, read_args) {
                Ok(res) => res,
                Err(_) => (channel_manager, None),
            }
        };

        let channel_manager = Arc::new(channel_manager);

        let peer_manager = Arc::new(PeerManager::new(
            MessageHandler {
                chan_handler: channel_manager.clone(),
                route_handler: channel_manager.clone(),
            },
            keys_manager.get_node_secret(),
            &logger,
        ));

        Ok(Self {
            network,
            keys_manager,
            channel_manager,
            peer_manager,
            bitcoin_support,
        })
    }

    pub async fn create_invoice(&self, amount_msat: u64, description: &str) -> Result<Invoice, Box<dyn Error>> {
        let currency = match self.network {
            BitcoinNetwork::Bitcoin => Currency::Bitcoin,
            BitcoinNetwork::Testnet => Currency::BitcoinTestnet,
            _ => return Err("Unsupported network".into()),
        };

        let invoice = Invoice::new(
            currency,
            amount_msat,
            description,
            None,
            None,
        )?;

        info!("Created Lightning invoice: {}", invoice.to_string());
        Ok(invoice)
    }

    pub async fn pay_invoice(&self, invoice: &Invoice) -> Result<(), Box<dyn Error>> {
        let payment_hash = invoice.payment_hash();
        let route = self.find_route(invoice.payee_pub_key(), invoice.amount_milli_satoshis().unwrap())?;

        self.channel_manager.send_payment(&route, payment_hash)?;
        info!("Payment sent for invoice: {}", invoice.to_string());
        Ok(())
    }

    pub async fn open_channel(&self, node_pubkey: &[u8], channel_value_satoshis: u64) -> Result<(), Box<dyn Error>> {
        let node_id = PublicKey::from_slice(node_pubkey)?;
        self.channel_manager.create_channel(node_id, channel_value_satoshis, 0, 0, None)?;
        info!("Channel opening initiated with node: {:?}", node_id);
        Ok(())
    }

    pub async fn close_channel(&self, channel_id: &[u8]) -> Result<(), Box<dyn Error>> {
        let channel_id = ChannelId::from_bytes(channel_id);
        self.channel_manager.close_channel(&channel_id)?;
        info!("Channel closure initiated for channel: {:?}", channel_id);
        Ok(())
    }

    pub async fn get_node_info(&self) -> Result<String, Box<dyn Error>> {
        let node_id = self.keys_manager.get_node_id();
        let channels = self.channel_manager.list_channels();
        let info = format!("Node ID: {:?}\nNumber of channels: {}", node_id, channels.len());
        Ok(info)
    }

    async fn find_route(&self, target: PublicKey, amount_msat: u64) -> Result<Route, Box<dyn Error>> {
        // Implement route finding logic here
        unimplemented!("Route finding not implemented")
    }
}
