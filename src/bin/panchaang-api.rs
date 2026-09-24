use axum::extract::Query;
use axum::{extract::Json, routing::{get, post}, Router};
use panchaang_engine::geo::cities::{all_cities, lookup_city};
use panchaang_engine::panchaang::forward::calculate_panchaang;
use panchaang_engine::types::{MonthSystem, PanchaangOutput, PanchangInput};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CityQuery {
    q: Option<String>,
    country: Option<String>,
    province: Option<String>,
}

#[derive(Deserialize)]
struct PanchangRequest {
    date_time: String,
    /// Raw coordinates. Required unless `city` is set.
    latitude: Option<f64>,
    longitude: Option<f64>,
    /// Gazetteer name, e.g. "Ahmedabad". Looked up in src/geo/cities.rs.
    city: Option<String>,
    country: Option<String>,
    province: Option<String>,
    /// "amanta" (default) or "purnimanta". Does not change tithi; only month naming.
    month_system: Option<String>,
    /// "vikrama" or "shaka". Year label only.
    era: Option<String>,
}

#[derive(Serialize)]
struct PanchangResponse {
    #[serde(flatten)]
    panchang: PanchaangOutput,
    month_system: MonthSystem,
    era: String,
    place: Option<Place>,
}

#[derive(Serialize)]
struct Place {
    name: String,
    province: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

async fn cities(Query(q): Query<CityQuery>) -> Json<serde_json::Value> {
    let list = if let Some(name) = q.q.as_deref() {
        lookup_city(name, q.country.as_deref(), q.province.as_deref())
    } else {
        all_cities().to_vec()
    };
    Json(serde_json::json!({ "count": list.len(), "cities": list }))
}

async fn panchang(
    Json(req): Json<PanchangRequest>,
) -> Result<Json<PanchangResponse>, (axum::http::StatusCode, String)> {
    let (lat, lon, place) = if let Some(name) = req.city.as_deref() {
        let hits = lookup_city(name, req.country.as_deref(), req.province.as_deref());
        let c = hits.into_iter().next().ok_or((
            axum::http::StatusCode::NOT_FOUND,
            format!("unknown city: {}", name),
        ))?;
        (
            c.latitude,
            c.longitude,
            Some(Place {
                name: c.name.to_string(),
                province: c.province.to_string(),
                country: c.country.to_string(),
                latitude: c.latitude,
                longitude: c.longitude,
            }),
        )
    } else {
        let lat = req.latitude.ok_or((
            axum::http::StatusCode::BAD_REQUEST,
            "latitude or city required".into(),
        ))?;
        let lon = req.longitude.ok_or((
            axum::http::StatusCode::BAD_REQUEST,
            "longitude or city required".into(),
        ))?;
        (lat, lon, None)
    };

    let dt = chrono::DateTime::parse_from_rfc3339(&req.date_time)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?
        .with_timezone(&chrono::Utc);
    let input = PanchangInput {
        date_time: dt,
        latitude: lat,
        longitude: lon,
        elevation_meters: None,
        ayanamsa_id: 1,
    };
    let out = calculate_panchaang(&input)
        .map_err(|e| (axum::http::StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;

    let month_system = match req.month_system.as_deref().map(|s| s.to_lowercase()).as_deref() {
        Some("purnimanta") => MonthSystem::Purnimanta,
        _ => MonthSystem::Amanta,
    };
    let era = match req.era.as_deref().map(|s| s.to_lowercase()).as_deref() {
        Some("shaka") => "shaka",
        _ => "vikrama",
    };

    Ok(Json(PanchangResponse {
        panchang: out,
        month_system,
        era: era.into(),
        place,
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/v1/panchang", post(panchang))
        .route("/v1/cities", get(cities));
    let bind = std::env::var("PANCHAANG_BIND").unwrap_or_else(|_| "127.0.0.1:8088".into());
    let listener = tokio::net::TcpListener::bind(&bind).await.expect("bind");
    eprintln!("panchaang-api on http://{bind}  POST /v1/panchang  GET /v1/cities");
    axum::serve(listener, app).await.expect("server");
}
