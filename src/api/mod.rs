use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use serde::Serialize;
use crate::ml::{MLModel, MLInput, MLOutput};
use crate::ai::AIModule;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::rate_limiter::RateLimiter;

async fn execute_high_volume_trade(
    data: web::Data<HighVolumeTrading>,
    ai_module: web::Data<Arc<Mutex<AIModule>>>
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
    
    let mut ai_module = ai_module.lock().await;
    let prediction = ai_module.predict(&ml_input).await;
    let training_data = vec![ml_input];
    if let Err(e) = ai_module.train(&training_data).await {
        log::error!("Error training ML model: {}", e);
    }
    let prediction_result = prediction;
    
    match prediction_result {
        Ok(prediction) => log::info!("ML prediction for trade: {:?}", prediction),
        Err(e) => log::error!("Error during ML prediction: {}", e),
    }
    
    HttpResponse::Ok().json(trade_params)
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

pub async fn start_api_server(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/analytics").to(get_advanced_analytics))
            .service(web::resource("/trade").to(execute_high_volume_trade))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

struct ApiHandler {
    rate_limiter: Arc<RateLimiter>,
}

impl ApiHandler {
    pub fn new(rate_limiter: Arc<RateLimiter>) -> Self {
        ApiHandler { rate_limiter }
    }

    pub async fn rate_limit_middleware(&self, req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
        if self.is_rate_limited(&req).await {
            return Ok(self.rate_limit_exceeded_response());
        }
        self.pass_request(body)
    }

    async fn is_rate_limited(&self, req: &HttpRequest) -> bool {
        let identifier = self.get_identifier(req).await;
        self.rate_limiter.is_limited(&identifier).await
    }

    fn rate_limit_exceeded_response(&self) -> HttpResponse {
        HttpResponse::TooManyRequests().json({
            "error": "Rate limit exceeded",
            "retry_after": 60 // Suggest retry after 60 seconds
        })
    }

    fn pass_request(&self, body: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
        Ok(HttpResponse::Ok().body(body))
    }

    async fn get_identifier(&self, req: &HttpRequest) -> String {
        // Implement logic to get a unique identifier (IP, wallet address, app ID, etc.)
        req.connection_info().realip_remote_addr()
            .unwrap_or("unknown")
            .to_string()
    }
}

// Wrap each endpoint with rate limiting middleware
// This macro takes an endpoint handler and wraps it with rate limiting logic.
macro_rules! rate_limited_endpoint {
    ($handler:expr) => {
        |api_handler: web::Data<ApiHandler>, req: HttpRequest, body: web::Bytes| async move {
            if api_handler.is_rate_limited(&req).await {
                return Ok(api_handler.rate_limit_exceeded_response());
            }
            $handler(req, body).await
        }
    };
}

// Example of using the macro for an endpoint
// Removed duplicate definition of get_advanced_analytics

pub fn config(cfg: &mut web::ServiceConfig) {
    let api_handler = web::Data::new(ApiHandler::new(Arc::new(RateLimiter::new())));
    cfg.app_data(api_handler.clone())
        .route("/analytics", web::post().to(rate_limited_endpoint!(get_advanced_analytics)))
        // Add other routes here, wrapped with rate_limited_endpoint! macro
        .route("/trade", web::post().to(rate_limited_endpoint!(execute_high_volume_trade)));
}