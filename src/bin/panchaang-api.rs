use axum::{extract::Json, routing::post, Router};
use panchaang_engine::panchaang::forward::calculate_panchaang;
use panchaang_engine::types::{PanchaangOutput, PanchangInput};
use serde::Deserialize;

#[derive(Deserialize)]
struct PanchangRequest {
    date_time: String,
    latitude: f64,
    longitude: f64,
}

async fn panchang(
    Json(req): Json<PanchangRequest>,
) -> Result<Json<PanchaangOutput>, (axum::http::StatusCode, String)> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.date_time)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?
        .with_timezone(&chrono::Utc);
    let input = PanchangInput {
        date_time: dt,
        latitude: req.latitude,
        longitude: req.longitude,
        elevation_meters: None,
        ayanamsa_id: 1,
    };
    calculate_panchaang(&input)
        .map(Json)
        .map_err(|e| (axum::http::StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/v1/panchang", post(panchang));
    let bind = std::env::var("PANCHAANG_BIND").unwrap_or_else(|_| "127.0.0.1:8088".into());
    let listener = tokio::net::TcpListener::bind(&bind).await.expect("bind");
    eprintln!("panchaang-api listening on http://{bind}  POST /v1/panchang");
    axum::serve(listener, app).await.expect("server");
}
