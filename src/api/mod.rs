use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::ml::{MLModel, MLInput, MLOutput};
use crate::ai::AIModule;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::rate_limiter::RateLimiter;

async fn get_advanced_analytics(
    data: web::Data<AdvancedAnalytics>,
    ai_module: web::Data<Arc<Mutex<AIModule>>>
) -> impl Responder {
    let analytics_params = AnalyticsParams {
        time_range: data.time_range.clone(),
        metrics: data.metrics.clone(),
        aggregation_level: data.aggregation_level.clone(),
    };
    
    // Feedback to ML model
    let ml_input = MLInput {
        analytics_request: analytics_params.clone(),
        // Add other relevant input data
    };
    
    if let Ok(prediction) = ai_module.lock().await.predict(&ml_input).await {
        // Use prediction to potentially adjust analytics parameters
        // For simplicity, we're just logging it here
        log::info!("ML prediction for analytics: {:?}", prediction);
    }
    
    // Train the model with this interaction
    let training_data = vec![ml_input];
    if let Err(e) = ai_module.lock().await.train(&training_data).await {
        log::error!("Error training ML model: {}", e);
    }
    
    web::Json(analytics_params)
}

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
    
    if let Ok(prediction) = ai_module.lock().await.predict(&ml_input).await {
        // Use prediction to potentially adjust trade parameters
        // For simplicity, we're just logging it here
        log::info!("ML prediction for trade: {:?}", prediction);
    }
    
    // Train the model with this interaction
    let training_data = vec![ml_input];
    if let Err(e) = ai_module.lock().await.train(&training_data).await {
        log::error!("Error training ML model: {}", e);
    }
    
    web::Json(trade_params)
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
<<<<<<< HEAD
}

pub struct ApiHandler {
    rate_limiter: Arc<RateLimiter>,
}

impl ApiHandler {
    pub fn new(rate_limiter: Arc<RateLimiter>) -> Self {
        ApiHandler { rate_limiter }
    }

    pub async fn rate_limit_middleware(&self, req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
        let identifier = self.get_identifier(&req);
        if !self.rate_limiter.check_rate_limit(&identifier).await {
            return Ok(HttpResponse::TooManyRequests().json({
                "error": "Rate limit exceeded",
                "retry_after": 60 // Suggest retry after 60 seconds
            }));
        }
        // If rate limit is not exceeded, pass the request to the next handler
        Ok(HttpResponse::Ok().body(body))
    }

    fn get_identifier(&self, req: &HttpRequest) -> String {
        // Implement logic to get a unique identifier (IP, wallet address, app ID, etc.)
        req.connection_info().realip_remote_addr()
            .unwrap_or("unknown")
            .to_string()
    }
}

// Wrap each endpoint with rate limiting middleware
macro_rules! rate_limited_endpoint {
    ($handler:expr) => {
        |api_handler: web::Data<ApiHandler>, req: HttpRequest, body: web::Bytes| async move {
            match api_handler.rate_limit_middleware(req, body).await {
                Ok(HttpResponse::Ok(_)) => $handler.await,
                Ok(response) => response,
                Err(e) => HttpResponse::InternalServerError().json({"error": e.to_string()}),
            }
        }
    };
}

// Example of using the macro for an endpoint
async fn get_advanced_analytics(data: web::Json<AnalyticsParams>) -> impl Responder {
    // Implementation...
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let api_handler = web::Data::new(ApiHandler::new(Arc::new(RateLimiter::new())));
    cfg.app_data(api_handler.clone())
        .route("/analytics", web::post().to(rate_limited_endpoint!(get_advanced_analytics)))
        // Add other routes here, wrapped with rate_limited_endpoint! macro
=======
>>>>>>> 1b4f7ce (Align project structure with updated architecture)
}