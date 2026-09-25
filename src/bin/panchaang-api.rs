use axum::extract::Query;
use axum::http::{HeaderValue, Method};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::{extract::Json, routing::{get, post}, Router};
use chrono::{Datelike, NaiveDate, Utc};
use panchaang_engine::calendar::month_cells;
use panchaang_engine::geo::cities::{all_cities, lookup_city};
use panchaang_engine::panchaang::forward::calculate_panchaang;
use panchaang_engine::reminders::{next_tithi_days, Paksha as _};
use panchaang_engine::types::{MonthSystem, PanchaangOutput, PanchangInput, Paksha};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CityQuery {
    q: Option<String>,
    country: Option<String>,
    province: Option<String>,
}

#[derive(Deserialize)]
struct PanchangRequest {
    date_time: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    city: Option<String>,
    country: Option<String>,
    province: Option<String>,
    month_system: Option<String>,
    era: Option<String>,
    year: Option<i32>,
    month: Option<u32>,
    paksha: Option<String>,
    tithi: Option<u8>,
    days: Option<i64>,
}

#[derive(Serialize)]
struct Place {
    name: String,
    province: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize)]
struct PanchangResponse {
    #[serde(flatten)]
    panchang: PanchaangOutput,
    month_system: MonthSystem,
    era: String,
    place: Option<Place>,
}

fn resolve_place(
    city: Option<&str>,
    country: Option<&str>,
    province: Option<&str>,
    latitude: Option<f64>,
    longitude: Option<f64>,
) -> Result<(f64, f64, Option<Place>), (axum::http::StatusCode, String)> {
    if let Some(name) = city {
        let hits = lookup_city(name, country, province);
        let c = hits.into_iter().next().ok_or((
            axum::http::StatusCode::NOT_FOUND,
            format!("unknown city: {}", name),
        ))?;
        return Ok((
            c.latitude,
            c.longitude,
            Some(Place {
                name: c.name.to_string(),
                province: c.province.to_string(),
                country: c.country.to_string(),
                latitude: c.latitude,
                longitude: c.longitude,
            }),
        ));
    }
    let lat = latitude.ok_or((
        axum::http::StatusCode::BAD_REQUEST,
        "latitude or city required".into(),
    ))?;
    let lon = longitude.ok_or((
        axum::http::StatusCode::BAD_REQUEST,
        "longitude or city required".into(),
    ))?;
    Ok((lat, lon, None))
}

fn parse_meta(month_system: Option<&str>, era: Option<&str>) -> (MonthSystem, String) {
    let ms = match month_system.map(|s| s.to_lowercase()).as_deref() {
        Some("purnimanta") => MonthSystem::Purnimanta,
        _ => MonthSystem::Amanta,
    };
    let era = match era.map(|s| s.to_lowercase()).as_deref() {
        Some("shaka") => "shaka",
        _ => "vikrama",
    };
    (ms, era.into())
}

async fn cities(Query(q): Query<CityQuery>) -> Json<serde_json::Value> {
    let list = if let Some(name) = q.q.as_deref() {
        lookup_city(name, q.country.as_deref(), q.province.as_deref())
    } else {
        all_cities().to_vec()
    };
    Json(serde_json::json!({ "count": list.len(), "cities": list }))
}

async fn panchang_post(
    Json(req): Json<PanchangRequest>,
) -> Result<Json<PanchangResponse>, (axum::http::StatusCode, String)> {
    panchang_core(req)
}

async fn panchang_get(
    Query(req): Query<PanchangRequest>,
) -> Result<Json<PanchangResponse>, (axum::http::StatusCode, String)> {
    panchang_core(req)
}

fn panchang_core(req: PanchangRequest) -> Result<Json<PanchangResponse>, (axum::http::StatusCode, String)> {
    let (lat, lon, place) = resolve_place(
        req.city.as_deref(),
        req.country.as_deref(),
        req.province.as_deref(),
        req.latitude,
        req.longitude,
    )?;
    let dt = if let Some(s) = req.date_time.as_deref() {
        chrono::DateTime::parse_from_rfc3339(s)
            .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?
            .with_timezone(&Utc)
    } else {
        Utc::now()
    };
    let input = PanchangInput {
        date_time: dt,
        latitude: lat,
        longitude: lon,
        elevation_meters: None,
        ayanamsa_id: 1,
    };
    let out = calculate_panchaang(&input)
        .map_err(|e| (axum::http::StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let (month_system, era) = parse_meta(req.month_system.as_deref(), req.era.as_deref());
    Ok(Json(PanchangResponse {
        panchang: out,
        month_system,
        era,
        place,
    }))
}

async fn calendar(
    Query(req): Query<PanchangRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let (lat, lon, place) = resolve_place(
        req.city.as_deref(),
        req.country.as_deref(),
        req.province.as_deref(),
        req.latitude,
        req.longitude,
    )?;
    let now = Utc::now();
    let year = req.year.unwrap_or(now.year());
    let month = req.month.unwrap_or(now.month());
    let days = month_cells(year, month, lat, lon)
        .map_err(|e| (axum::http::StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    Ok(Json(serde_json::json!({
        "year": year,
        "month": month,
        "place": place,
        "days": days
    })))
}

async fn next_tithi(
    Query(req): Query<PanchangRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let (lat, lon, place) = resolve_place(
        req.city.as_deref(),
        req.country.as_deref(),
        req.province.as_deref(),
        req.latitude,
        req.longitude,
    )?;
    let paksha = match req.paksha.as_deref().map(|s| s.to_lowercase()).as_deref() {
        Some("krishna") => Paksha::Krishna,
        _ => Paksha::Shukla,
    };
    let tithi = req.tithi.unwrap_or(1);
    let horizon = req.days.unwrap_or(40);
    let from = Utc::now().date_naive();
    let hits = next_tithi_days(from, lat, lon, paksha, tithi, horizon)
        .map_err(|e| (axum::http::StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    Ok(Json(serde_json::json!({
        "place": place,
        "paksha": paksha,
        "tithi": tithi,
        "hits": hits
    })))
}

async fn cors(req: axum::extract::Request, next: Next) -> Response {
    if req.method() == Method::OPTIONS {
        let mut res = Response::new(axum::body::Body::empty());
        let h = res.headers_mut();
        h.insert("access-control-allow-origin", HeaderValue::from_static("*"));
        h.insert("access-control-allow-methods", HeaderValue::from_static("GET,POST,OPTIONS"));
        h.insert("access-control-allow-headers", HeaderValue::from_static("content-type"));
        return res;
    }
    let mut res = next.run(req).await;
    res.headers_mut()
        .insert("access-control-allow-origin", HeaderValue::from_static("*"));
    res
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/v1/panchang", post(panchang_post).get(panchang_get))
        .route("/v1/calendar", get(calendar))
        .route("/v1/next-tithi", get(next_tithi))
        .route("/v1/cities", get(cities))
        .layer(middleware::from_fn(cors));
    let bind = std::env::var("PANCHAANG_BIND").unwrap_or_else(|_| "127.0.0.1:8088".into());
    let listener = tokio::net::TcpListener::bind(&bind).await.expect("bind");
    eprintln!("panchaang-api http://{bind}");
    axum::serve(listener, app).await.expect("server");
}
