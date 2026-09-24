use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct City {
    pub name: &'static str,
    pub province: &'static str,
    pub country: &'static str,
    pub country_code: &'static str,
    pub latitude: f64,
    pub longitude: f64,
    pub tz_offset_hours: f64,
}

/// Gazetteer of major places. Not the calculation — any lat/lon still works
/// without appearing here. Add rows as paying regions appear.
const CITIES: &[City] = &[
    // India — certified + state / metro
    City { name: "Delhi", province: "Delhi", country: "India", country_code: "IN", latitude: 28.6139, longitude: 77.2090, tz_offset_hours: 5.5 },
    City { name: "New Delhi", province: "Delhi", country: "India", country_code: "IN", latitude: 28.6139, longitude: 77.2090, tz_offset_hours: 5.5 },
    City { name: "Mumbai", province: "Maharashtra", country: "India", country_code: "IN", latitude: 19.0760, longitude: 72.8777, tz_offset_hours: 5.5 },
    City { name: "Ujjain", province: "Madhya Pradesh", country: "India", country_code: "IN", latitude: 23.1765, longitude: 75.7849, tz_offset_hours: 5.5 },
    City { name: "Jaipur", province: "Rajasthan", country: "India", country_code: "IN", latitude: 26.9124, longitude: 75.7873, tz_offset_hours: 5.5 },
    City { name: "Ahmedabad", province: "Gujarat", country: "India", country_code: "IN", latitude: 23.0225, longitude: 72.5714, tz_offset_hours: 5.5 },
    City { name: "Bharuch", province: "Gujarat", country: "India", country_code: "IN", latitude: 21.7051, longitude: 72.9959, tz_offset_hours: 5.5 },
    City { name: "Gandhinagar", province: "Gujarat", country: "India", country_code: "IN", latitude: 23.2156, longitude: 72.6369, tz_offset_hours: 5.5 },
    City { name: "Surat", province: "Gujarat", country: "India", country_code: "IN", latitude: 21.1702, longitude: 72.8311, tz_offset_hours: 5.5 },
    City { name: "Vadodara", province: "Gujarat", country: "India", country_code: "IN", latitude: 22.3072, longitude: 73.1812, tz_offset_hours: 5.5 },
    City { name: "Rajkot", province: "Gujarat", country: "India", country_code: "IN", latitude: 22.3039, longitude: 70.8022, tz_offset_hours: 5.5 },
    City { name: "Bengaluru", province: "Karnataka", country: "India", country_code: "IN", latitude: 12.9716, longitude: 77.5946, tz_offset_hours: 5.5 },
    City { name: "Chennai", province: "Tamil Nadu", country: "India", country_code: "IN", latitude: 13.0827, longitude: 80.2707, tz_offset_hours: 5.5 },
    City { name: "Kolkata", province: "West Bengal", country: "India", country_code: "IN", latitude: 22.5726, longitude: 88.3639, tz_offset_hours: 5.5 },
    City { name: "Hyderabad", province: "Telangana", country: "India", country_code: "IN", latitude: 17.3850, longitude: 78.4867, tz_offset_hours: 5.5 },
    City { name: "Pune", province: "Maharashtra", country: "India", country_code: "IN", latitude: 18.5204, longitude: 73.8567, tz_offset_hours: 5.5 },
    City { name: "Lucknow", province: "Uttar Pradesh", country: "India", country_code: "IN", latitude: 26.8467, longitude: 80.9462, tz_offset_hours: 5.5 },
    City { name: "Varanasi", province: "Uttar Pradesh", country: "India", country_code: "IN", latitude: 25.3176, longitude: 82.9739, tz_offset_hours: 5.5 },
    City { name: "Patna", province: "Bihar", country: "India", country_code: "IN", latitude: 25.5941, longitude: 85.1376, tz_offset_hours: 5.5 },
    City { name: "Bhopal", province: "Madhya Pradesh", country: "India", country_code: "IN", latitude: 23.2599, longitude: 77.4126, tz_offset_hours: 5.5 },
    City { name: "Indore", province: "Madhya Pradesh", country: "India", country_code: "IN", latitude: 22.7196, longitude: 75.8577, tz_offset_hours: 5.5 },
    City { name: "Nagpur", province: "Maharashtra", country: "India", country_code: "IN", latitude: 21.1458, longitude: 79.0882, tz_offset_hours: 5.5 },
    City { name: "Chandigarh", province: "Chandigarh", country: "India", country_code: "IN", latitude: 30.7333, longitude: 76.7794, tz_offset_hours: 5.5 },
    City { name: "Amritsar", province: "Punjab", country: "India", country_code: "IN", latitude: 31.6340, longitude: 74.8723, tz_offset_hours: 5.5 },
    City { name: "Srinagar", province: "Jammu and Kashmir", country: "India", country_code: "IN", latitude: 34.0837, longitude: 74.7973, tz_offset_hours: 5.5 },
    City { name: "Guwahati", province: "Assam", country: "India", country_code: "IN", latitude: 26.1445, longitude: 91.7362, tz_offset_hours: 5.5 },
    City { name: "Bhubaneswar", province: "Odisha", country: "India", country_code: "IN", latitude: 20.2961, longitude: 85.8245, tz_offset_hours: 5.5 },
    City { name: "Thiruvananthapuram", province: "Kerala", country: "India", country_code: "IN", latitude: 8.5241, longitude: 76.9366, tz_offset_hours: 5.5 },
    City { name: "Kochi", province: "Kerala", country: "India", country_code: "IN", latitude: 9.9312, longitude: 76.2673, tz_offset_hours: 5.5 },
    City { name: "Panaji", province: "Goa", country: "India", country_code: "IN", latitude: 15.4909, longitude: 73.8278, tz_offset_hours: 5.5 },
    City { name: "Ranchi", province: "Jharkhand", country: "India", country_code: "IN", latitude: 23.3441, longitude: 85.3096, tz_offset_hours: 5.5 },
    City { name: "Raipur", province: "Chhattisgarh", country: "India", country_code: "IN", latitude: 21.2514, longitude: 81.6296, tz_offset_hours: 5.5 },
    City { name: "Dehradun", province: "Uttarakhand", country: "India", country_code: "IN", latitude: 30.3165, longitude: 78.0322, tz_offset_hours: 5.5 },
    City { name: "Shimla", province: "Himachal Pradesh", country: "India", country_code: "IN", latitude: 31.1048, longitude: 77.1734, tz_offset_hours: 5.5 },
    City { name: "Itanagar", province: "Arunachal Pradesh", country: "India", country_code: "IN", latitude: 27.0844, longitude: 93.6053, tz_offset_hours: 5.5 },
    City { name: "Imphal", province: "Manipur", country: "India", country_code: "IN", latitude: 24.8170, longitude: 93.9368, tz_offset_hours: 5.5 },
    City { name: "Aizawl", province: "Mizoram", country: "India", country_code: "IN", latitude: 23.7271, longitude: 92.7176, tz_offset_hours: 5.5 },
    City { name: "Kohima", province: "Nagaland", country: "India", country_code: "IN", latitude: 25.6751, longitude: 94.1086, tz_offset_hours: 5.5 },
    City { name: "Agartala", province: "Tripura", country: "India", country_code: "IN", latitude: 23.8315, longitude: 91.2868, tz_offset_hours: 5.5 },
    City { name: "Shillong", province: "Meghalaya", country: "India", country_code: "IN", latitude: 25.5788, longitude: 91.8933, tz_offset_hours: 5.5 },
    City { name: "Gangtok", province: "Sikkim", country: "India", country_code: "IN", latitude: 27.3389, longitude: 88.6065, tz_offset_hours: 5.5 },
    City { name: "Port Blair", province: "Andaman and Nicobar", country: "India", country_code: "IN", latitude: 11.6234, longitude: 92.7265, tz_offset_hours: 5.5 },
    City { name: "Kavaratti", province: "Lakshadweep", country: "India", country_code: "IN", latitude: 10.5593, longitude: 72.6358, tz_offset_hours: 5.5 },
    City { name: "Puducherry", province: "Puducherry", country: "India", country_code: "IN", latitude: 11.9416, longitude: 79.8083, tz_offset_hours: 5.5 },
    City { name: "Leh", province: "Ladakh", country: "India", country_code: "IN", latitude: 34.1526, longitude: 77.5771, tz_offset_hours: 5.5 },
    // Neighbours
    City { name: "Kathmandu", province: "Bagmati", country: "Nepal", country_code: "NP", latitude: 27.7172, longitude: 85.3240, tz_offset_hours: 5.75 },
    City { name: "Colombo", province: "Western", country: "Sri Lanka", country_code: "LK", latitude: 6.9271, longitude: 79.8612, tz_offset_hours: 5.5 },
    City { name: "Dhaka", province: "Dhaka", country: "Bangladesh", country_code: "BD", latitude: 23.8103, longitude: 90.4125, tz_offset_hours: 6.0 },
    City { name: "Karachi", province: "Sindh", country: "Pakistan", country_code: "PK", latitude: 24.8607, longitude: 67.0011, tz_offset_hours: 5.0 },
    City { name: "Lahore", province: "Punjab", country: "Pakistan", country_code: "PK", latitude: 31.5204, longitude: 74.3587, tz_offset_hours: 5.0 },
    // Diaspora / world hubs
    City { name: "London", province: "England", country: "United Kingdom", country_code: "GB", latitude: 51.5074, longitude: -0.1278, tz_offset_hours: 0.0 },
    City { name: "New York", province: "New York", country: "United States", country_code: "US", latitude: 40.7128, longitude: -74.0060, tz_offset_hours: -5.0 },
    City { name: "Chicago", province: "Illinois", country: "United States", country_code: "US", latitude: 41.8781, longitude: -87.6298, tz_offset_hours: -6.0 },
    City { name: "San Francisco", province: "California", country: "United States", country_code: "US", latitude: 37.7749, longitude: -122.4194, tz_offset_hours: -8.0 },
    City { name: "Los Angeles", province: "California", country: "United States", country_code: "US", latitude: 34.0522, longitude: -118.2437, tz_offset_hours: -8.0 },
    City { name: "Toronto", province: "Ontario", country: "Canada", country_code: "CA", latitude: 43.6532, longitude: -79.3832, tz_offset_hours: -5.0 },
    City { name: "Vancouver", province: "British Columbia", country: "Canada", country_code: "CA", latitude: 49.2827, longitude: -123.1207, tz_offset_hours: -8.0 },
    City { name: "Sydney", province: "New South Wales", country: "Australia", country_code: "AU", latitude: -33.8688, longitude: 151.2093, tz_offset_hours: 10.0 },
    City { name: "Melbourne", province: "Victoria", country: "Australia", country_code: "AU", latitude: -37.8136, longitude: 144.9631, tz_offset_hours: 10.0 },
    City { name: "Singapore", province: "Singapore", country: "Singapore", country_code: "SG", latitude: 1.3521, longitude: 103.8198, tz_offset_hours: 8.0 },
    City { name: "Dubai", province: "Dubai", country: "United Arab Emirates", country_code: "AE", latitude: 25.2048, longitude: 55.2708, tz_offset_hours: 4.0 },
    City { name: "Doha", province: "Ad Dawhah", country: "Qatar", country_code: "QA", latitude: 25.2854, longitude: 51.5310, tz_offset_hours: 3.0 },
    City { name: "Nairobi", province: "Nairobi", country: "Kenya", country_code: "KE", latitude: -1.2921, longitude: 36.8219, tz_offset_hours: 3.0 },
    City { name: "Johannesburg", province: "Gauteng", country: "South Africa", country_code: "ZA", latitude: -26.2041, longitude: 28.0473, tz_offset_hours: 2.0 },
    City { name: "Port Louis", province: "Port Louis", country: "Mauritius", country_code: "MU", latitude: -20.1609, longitude: 57.5012, tz_offset_hours: 4.0 },
    City { name: "Tokyo", province: "Tokyo", country: "Japan", country_code: "JP", latitude: 35.6762, longitude: 139.6503, tz_offset_hours: 9.0 },
    City { name: "Paris", province: "Île-de-France", country: "France", country_code: "FR", latitude: 48.8566, longitude: 2.3522, tz_offset_hours: 1.0 },
    City { name: "Berlin", province: "Berlin", country: "Germany", country_code: "DE", latitude: 52.5200, longitude: 13.4050, tz_offset_hours: 1.0 },
    City { name: "Moscow", province: "Moscow", country: "Russia", country_code: "RU", latitude: 55.7558, longitude: 37.6173, tz_offset_hours: 3.0 },
    City { name: "São Paulo", province: "São Paulo", country: "Brazil", country_code: "BR", latitude: -23.5505, longitude: -46.6333, tz_offset_hours: -3.0 },
];

pub fn all_cities() -> &'static [City] {
    CITIES
}

/// Case-insensitive name match; optional country or province filter.
pub fn lookup_city(query: &str, country: Option<&str>, province: Option<&str>) -> Vec<City> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }
    CITIES
        .iter()
        .copied()
        .filter(|c| {
            let name_hit = c.name.to_lowercase().contains(&q);
            let country_ok = country.map(|x| c.country.eq_ignore_ascii_case(x) || c.country_code.eq_ignore_ascii_case(x)).unwrap_or(true);
            let prov_ok = province.map(|x| c.province.eq_ignore_ascii_case(x)).unwrap_or(true);
            name_hit && country_ok && prov_ok
        })
        .collect()
}
