use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::privacy::PrivacyModule;
use crate::bitcoin::BitcoinModule;
use bitcoin::{TxIn, TxOut, Transaction};

pub struct ApiServer {
    privacy_module: PrivacyModule,
    bitcoin_module: BitcoinModule,
}

impl ApiServer {
    pub fn new(privacy_module: PrivacyModule, bitcoin_module: BitcoinModule) -> Self {
        Self {
            privacy_module,
            bitcoin_module,
        }
    }

    pub async fn run(&self, host: &str, port: u16) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(web::resource("/health").to(health_check))
                .service(web::resource("/create_transaction").to(create_transaction))
                // Add more endpoints here
        })
        .bind((host, port))?
        .run()
        .await
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[derive(Deserialize)]
struct CreateTransactionRequest {
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
}

#[derive(Serialize)]
struct CreateTransactionResponse {
    transaction: Transaction,
}

async fn create_transaction(
    data: web::Json<CreateTransactionRequest>,
    api_server: web::Data<ApiServer>,
) -> impl Responder {
    let transaction = api_server.bitcoin_module.create_transaction(data.inputs.clone(), data.outputs.clone());
    HttpResponse::Ok().json(CreateTransactionResponse { transaction })
}