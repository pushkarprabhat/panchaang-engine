use chrono::Utc;
use panchaang_engine::panchaang;
use panchaang_engine::types::PanchangInput;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: panchaang <ISO_DATETIME> <lat> <lon>");
        std::process::exit(2);
    }
    let dt = match chrono::DateTime::parse_from_rfc3339(&args[1]) {
        Ok(d) => d.with_timezone(&Utc),
        Err(e) => {
            eprintln!("invalid datetime: {}", e);
            std::process::exit(2);
        }
    };
    let lat: f64 = args[2].parse().unwrap_or(0.0);
    let lon: f64 = args[3].parse().unwrap_or(0.0);
    let input = PanchangInput {
        date_time: dt,
        latitude: lat,
        longitude: lon,
        elevation_meters: None,
        ayanamsa_id: 1,
    };
    match panchaang::forward::calculate_panchaang(&input) {
        Ok(out) => {
            println!(
                "tithi={} paksha={:?} nakshatra={} yoga={} karana={}",
                out.tithi_number, out.paksha, out.nakshatra_index, out.yoga_index, out.karana_index
            );
            println!("sunrise={} sunset={}", out.sunrise, out.sunset);
            println!("tithi_start={} tithi_end={}", out.tithi_start, out.tithi_end);
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}
