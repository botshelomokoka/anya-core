/// The above Rust code defines API endpoints for processing enterprise data, retrieving revenue
/// insights, and getting machine learning predictions.
/// 
/// Returns:
/// 
/// The code snippet provided defines an Actix web API with endpoints for processing enterprise data and
/// retrieving revenue insights.
use crate::integration::unified_data_system::UnifiedDataSystem;
use actix_web::{web, HttpResponse, Scope};
use serde::{Serialize, Deserialize};

pub fn enterprise_api_scope() -> Scope {
    web::scope("/api/v1")
        .service(process_enterprise_data)
        .service(get_revenue_insights)
        .service(get_ml_predictions)
}

#[derive(Debug, Serialize)]
struct EnterpriseResponse {
    success: bool,
    data: serde_json::Value,
    revenue_metrics: RevenueMetrics,
    ml_insights: MLInsights,
}

#[post("/process")]
async fn process_enterprise_data(
    data: web::Json<UnifiedDataRecord>,
    system: web::Data<Arc<UnifiedDataSystem>>,
) -> HttpResponse {
    match system.process_data(data.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(EnterpriseResponse {
            success: true,
            data: serde_json::to_value(&result).unwrap(),
            revenue_metrics: result.revenue_metrics,
            ml_insights: result.ml_insights,
        }),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": e.to_string(),
        })),
    }
}

#[get("/insights")]
async fn get_revenue_insights(
    timeframe: web::Query<TimeFrameQuery>,
    system: web::Data<Arc<UnifiedDataSystem>>,
) -> HttpResponse {
    match system.get_revenue_insights(timeframe.into_inner().into()).await {
        Ok(insights) => HttpResponse::Ok().json(insights),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}
