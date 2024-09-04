//! This module provides a client interface for interacting with the Lightning Network using LND.

use std::error::Error;
use tonic_lnd::lnrpc::{
    AddInvoiceResponse, Channel, ChannelPoint, CloseChannelRequest, Invoice, ListChannelsResponse,
    OpenChannelRequest, PayReq, PaymentHash, SendResponse, WalletBalanceResponse,
};
use tonic_lnd::Client as LndClient;

/// A struct representing the Lightning Network client for LND
pub struct LightningNetworkClient {
    client: LndClient,
}

impl LightningNetworkClient {
    /// Creates a new LightningNetworkClient
    pub async fn new(host: &str, cert_path: &str, macaroon_path: &str) -> Result<Self, Box<dyn Error>> {
        let client = LndClient::new_from_cert(host, cert_path, macaroon_path).await?;
        Ok(Self { client })
    }

    /// Opens a Lightning Network channel with a specified node
    ///
    /// # Arguments
    ///
    /// * `node_pubkey` - The public key of the Lightning node to connect to
    /// * `funding_amount` - The amount of satoshis to fund the channel with
    /// * `push_sat` - (Optional) The amount of satoshis to push to the remote node
    ///
    /// # Returns
    ///
    /// The `ChannelPoint` object representing the opened channel
    pub async fn open_channel(&self, node_pubkey: &str, funding_amount: i64, push_sat: Option<i64>) -> Result<ChannelPoint, Box<dyn Error>> {
        let mut request = OpenChannelRequest::default();
        request.set_node_pubkey_string(node_pubkey.to_string());
        request.set_local_funding_amount(funding_amount);
        if let Some(push) = push_sat {
            request.set_push_sat(push);
        }

        let response = self.client.lightning().open_channel_sync(request).await?;
        Ok(response.into_inner())
    }

    /// Decodes a Lightning invoice (BOLT11 format) to get payment details
    ///
    /// # Arguments
    ///
    /// * `invoice` - The Lightning invoice string
    ///
    /// # Returns
    ///
    /// A `PayReq` object containing the decoded invoice details
    pub async fn decode_invoice(&self, invoice: &str) -> Result<PayReq, Box<dyn Error>> {
        let mut request = tonic_lnd::lnrpc::PayReqString::default();
        request.set_pay_req(invoice.to_string());

        let response = self.client.lightning().decode_pay_req(request).await?;
        Ok(response.into_inner())
    }

    /// Sends a Lightning Network payment
    ///
    /// # Arguments
    ///
    /// * `invoice` - The Lightning invoice to pay
    ///
    /// # Returns
    ///
    /// The `SendResponse` if successful
    pub async fn send_payment(&self, invoice: &str) -> Result<SendResponse, Box<dyn Error>> {
        let mut request = tonic_lnd::lnrpc::SendRequest::default();
        request.set_payment_request(invoice.to_string());

        let response = self.client.lightning().send_payment_sync(request).await?;
        Ok(response.into_inner())
    }

    /// Receives a Lightning Network payment by creating an invoice
    ///
    /// # Arguments
    ///
    /// * `amount_sat` - The amount to receive in satoshis
    /// * `memo` - A description for the invoice
    ///
    /// # Returns
    ///
    /// The `AddInvoiceResponse` containing the generated invoice
    pub async fn receive_payment(&self, amount_sat: i64, memo: &str) -> Result<AddInvoiceResponse, Box<dyn Error>> {
        let mut invoice = Invoice::default();
        invoice.set_value(amount_sat);
        invoice.set_memo(memo.to_string());

        let response = self.client.lightning().add_invoice(invoice).await?;
        Ok(response.into_inner())
    }

    /// Closes a Lightning Network channel
    ///
    /// # Arguments
    ///
    /// * `channel_point` - The funding outpoint of the channel to close
    /// * `force` - Whether to force close the channel
    ///
    /// # Returns
    ///
    /// A stream of close status update messages
    pub async fn close_channel(&self, channel_point: ChannelPoint, force: bool) -> Result<tonic::Response<tonic::Streaming<tonic_lnd::lnrpc::CloseStatusUpdate>>, Box<dyn Error>> {
        let mut request = CloseChannelRequest::default();
        request.set_channel_point(channel_point);
        request.set_force(force);

        let response = self.client.lightning().close_channel(request).await?;
        Ok(response)
    }

    /// Gets the current balance of the Lightning wallet
    ///
    /// # Returns
    ///
    /// A `WalletBalanceResponse` containing the confirmed and unconfirmed balance
    pub async fn get_balance(&self) -> Result<WalletBalanceResponse, Box<dyn Error>> {
        let request = tonic_lnd::lnrpc::WalletBalanceRequest::default();
        let response = self.client.lightning().wallet_balance(request).await?;
        Ok(response.into_inner())
    }

    /// Lists all open channels
    ///
    /// # Returns
    ///
    /// A `ListChannelsResponse` containing details of all open channels
    pub async fn list_channels(&self) -> Result<ListChannelsResponse, Box<dyn Error>> {
        let request = tonic_lnd::lnrpc::ListChannelsRequest::default();
        let response = self.client.lightning().list_channels(request).await?;
        Ok(response.into_inner())
    }
}
