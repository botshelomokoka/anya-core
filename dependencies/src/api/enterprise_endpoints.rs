use crate::{
    auth::{AuthManager, BlockchainAuth},
    integration::unified_data_system::UnifiedDataSystem,
    ml::enterprise_integration::MLEnterpriseIntegration,
};
use actix_web::{web, HttpResponse, Scope};
use serde::{Serialize, Deserialize};

pub fn enterprise_api_scope() -> Scope {
    web::scope("/api/v1/enterprise")
        .service(process_data)
        .service(get_insights)
        .service(get_predictions)
        .service(get_revenue_metrics)
        .service(analyze_market_data)
        .service(get_security_metrics)
}

#[post("/data/process")]
async fn process_data(
    data: web::Json<UnifiedDataRecord>,
    auth: web::Data<Arc<AuthManager>>,
    system: web::Data<Arc<UnifiedDataSystem>>,
) -> HttpResponse {
    // Verify authentication
    if !auth.verify(&data.credentials).await? {
        return HttpResponse::Unauthorized().finish();
    }

    // Process with revenue tracking
    let result = system.process_data_with_revenue(data.into_inner()).await?;
    
    HttpResponse::Ok().json(EnterpriseResponse {
        success: true,
        data: result.data,
        revenue_metrics: result.revenue_metrics,
        ml_insights: result.ml_insights,
        security_metrics: result.security_metrics,
    })
}

#[get("/insights/market")]
async fn analyze_market_data(
    params: web::Query<MarketAnalysisParams>,
    auth: web::Data<Arc<AuthManager>>,
    ml: web::Data<Arc<MLEnterpriseIntegration>>,
) -> HttpResponse {
    let analysis = ml.analyze_market_data(
        params.market_id,
        params.timeframe,
        params.indicators,
    ).await?;
    
    HttpResponse::Ok().json(analysis)
}

#[get("/metrics/revenue")]
async fn get_revenue_metrics(
    timeframe: web::Query<TimeFrameQuery>,
    auth: web::Data<Arc<AuthManager>>,
    system: web::Data<Arc<UnifiedDataSystem>>,
) -> HttpResponse {
    let metrics = system.get_detailed_revenue_metrics(
        timeframe.start_date,
        timeframe.end_date,
    ).await?;
    
    HttpResponse::Ok().json(metrics)
}

#[get("/metrics/security")]
async fn get_security_metrics(
    auth: web::Data<Arc<AuthManager>>,
    system: web::Data<Arc<UnifiedDataSystem>>,
) -> HttpResponse {
    let metrics = system.get_security_metrics().await?;
    HttpResponse::Ok().json(metrics)
}
