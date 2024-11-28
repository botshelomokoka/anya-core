//! Module documentation for $moduleName
//!
//! # Overview
//! This module is part of the Anya Core project, located at $modulePath.
//!
//! # Architecture
//! [Add module-specific architecture details]
//!
//! # API Reference
//! [Document public functions and types]
//!
//! # Usage Examples
//! `
ust
//! // Add usage examples
//! `
//!
//! # Error Handling
//! This module uses proper error handling with Result types.
//!
//! # Security Considerations
//! [Document security features and considerations]
//!
//! # Performance
//! [Document performance characteristics]

use std::error::Error;
use crate::chain_support::{ChainSupport, BitcoinSupport};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::error;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;
use crate::config::PyConfig;
use crate::api_doc::ApiDoc;
use bitcoin::network::constants::Network;

#[derive(Serialize, Deserialize, ToSchema)]
struct CreateWalletRequest {
    name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct SendTransactionRequest {
    to: String,
    amount: u64,
}

#[utoipa::path(
    post,
    path = "/api/bitcoin/create_wallet",
    request_body = CreateWalletRequest,
    responses(
        (status = 200, description = "Wallet created successfully"),
        (status = 500, description = "Internal server error")
    )
)]
async fn handle_create_wallet(bitcoin_support: web::Data<BitcoinSupport>, req: web::Json<CreateWalletRequest>) -> impl Responder  -> Result<(), Box<dyn Error>> {
    match bitcoin_support.create_wallet(&req.name).await {
        Ok(_) => HttpResponse::Ok().body("Wallet created successfully"),
        Err(e) => {
            error!("Error creating wallet: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        },
    }
}

#[utoipa::path(
    post,
    path = "/api/bitcoin/send_transaction",
    request_body = SendTransactionRequest,
    responses(
        (status = 200, description = "Transaction sent successfully", body = String),
        (status = 500, description = "Internal server error")
    )
)]
async fn handle_send_transaction(bitcoin_support: web::Data<BitcoinSupport>, req: web::Json<SendTransactionRequest>) -> impl Responder  -> Result<(), Box<dyn Error>> {
    match bitcoin_support.send_transaction(&req.to, req.amount).await {
        Ok(txid) => HttpResponse::Ok().body(txid),
        Err(e) => {
            error!("Error sending transaction: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        },
    }
}

pub async fn start_api_server(config: PyConfig) -> std::io::Result<()>  -> Result<(), Box<dyn Error>> {
    info!("Starting API server");
    let openapi = ApiDoc::openapi();
    let bitcoin_support = web::Data::new(BitcoinSupport::new(
        &config.bitcoin_rpc_url,
        &config.bitcoin_rpc_user,
        &config.bitcoin_rpc_pass,
        Network::Bitcoin,
    )?);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(bitcoin_support.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
            .route("/api/analysis", web::post().to(handle_analysis))
            .route("/api/verify_transaction", web::post().to(handle_verify_transaction))
            .route("/api/bitcoin/create_wallet", web::post().to(handle_create_wallet))
            .route("/api/bitcoin/send_transaction", web::post().to(handle_send_transaction))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

