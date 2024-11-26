use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ai::AIModule;
use crate::ml::{MLInput, MLModel, MLOutput};
use crate::rate_limiter::RateLimiter;
// Removed duplicate use statement
async fn execute_high_volume_trade(
    data: web::Data<HighVolumeTrading>,
    ai_module: web::Data<Arc<Mutex<AIModule>>>,
) -> impl Responder {
    let trade_params = TradeParams {
        asset: data.asset.clone(),
        volume: data.volume,
        price: data.price,
        trade_type: data.trade_type.clone(),
    };
    
    // Feedback to ML model
    let ml_input = MLInput {
        trade_request: trade_params.clone(),
        // Add other relevant input data
    };
    
    let prediction;
    {
        let mut ai_module = ai_module.lock().await;
        prediction = ai_module.predict(&ml_input).await;
    }
    
    let training_data = vec![ml_input];
    {
        let mut ai_module = ai_module.lock().await;
        if let Err(e) = ai_module.train(&training_data).await {
            log::error!("Error training ML model: {}", e);
        }
    }
    
    match prediction {
        Ok(prediction) => log::info!("ML prediction for trade: {:?}", prediction),
        Err(e) => log::error!("Error during ML prediction: {}", e),
    }
    
    match prediction {
        Ok(prediction) => HttpResponse::Ok().json(prediction),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error during ML prediction: {}", e)),
    }
}

#[derive(Serialize, Clone)]
struct AnalyticsParams {
    time_range: String,
    metrics: Vec<String>,
    aggregation_level: String,
}

#[derive(Serialize, Clone)]
struct TradeParams {
    asset: String,
    volume: f64,
    price: f64,
    trade_type: String,
}

#[derive(Clone)]
struct HighVolumeTrading {
    asset: String,
    volume: f64,
    price: f64,
    trade_type: String,
}

pub async fn start_api_server(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/analytics").to(get_advanced_analytics))
            .service(web::resource("/trade").to(execute_high_volume_trade))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
// anya-enterprise/src/api.rs
use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    amount: f64,
    flagged: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    balance: f64,
    alert: bool,
}

#[post("/transaction")]
async fn create_transaction(transaction: web::Json<Transaction>) -> impl Responder {
    // Process transaction
    web::Json(transaction.into_inner())
}

#[get("/account/{id}")]
async fn get_account(web::Path(id): web::Path<u32>) -> impl Responder {
    // Retrieve account by ID
    web::Json(Account {
        balance: 100.0,
        alert: false,
    })
}

#[actix_web::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_transaction)
            .service(get_account)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub struct ApiHandler {
    rate_limiter: Arc<RateLimiter>,
}

impl ApiHandler {
    pub fn new(rate_limiter: Arc<RateLimiter>) -> Self {
        Self { rate_limiter }
    }

    pub async fn handle_request<F, Fut>(&self, f: F) -> Result<impl Responder>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<impl Responder>>,
    {
        self.rate_limiter.acquire(1).await?;
        f().await
    }
}

// Rate limiting macro
macro_rules! rate_limited_endpoint {
    ($handler:expr) => {
        |req, body, api_handler: web::Data<ApiHandler>| async move {
            api_handler.handle_request(|| async { $handler(req, body).await }).await
        }
    };
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let api_handler = web::Data::new(ApiHandler::new(
        Arc::new(RateLimiter::new(100, 10.0))
    ));

    cfg.app_data(api_handler.clone())
        .route("/analytics", web::post().to(rate_limited_endpoint!(get_advanced_analytics)));
}
