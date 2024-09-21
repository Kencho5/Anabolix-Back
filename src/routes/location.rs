use crate::{imports::*, utils::ip_struct};
use maxminddb::geoip2;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Serialize)]
struct LocationData {
    ip: String,
    city: Option<String>,
    country: Option<String>,
    language: Option<String>,
}

impl LocationData {
    fn from_geoip(location: geoip2::City, ip: String) -> Self {
        let city = location
            .city
            .as_ref()
            .and_then(|city| city.names.as_ref())
            .and_then(|names| names.get("en").map(|s| s.to_string()));

        let country = location
            .country
            .as_ref()
            .and_then(|country| country.names.as_ref())
            .and_then(|names| names.get("en").map(|s| s.to_string()));

        let language = location
            .country
            .as_ref()
            .and_then(|country| country.iso_code.map(|code| code.to_string().to_lowercase()));

        LocationData {
            ip,
            city,
            country,
            language,
        }
    }
}

pub async fn location_handler(req: Request<AppState>) -> tide::Result {
    let mut response = Response::builder(200)
        .content_type("application/json")
        .build();

    let state = req.state();
    let ip: IpAddr = match FromStr::from_str("188.169.122.164") {
        Ok(ip) => ip,
        Err(_) => {
            response.set_status(404);
            return Ok(response);
        }
    };
    let location: LocationData = match state.ip_reader.lookup::<geoip2::City>(ip) {
        Ok(city) => LocationData::from_geoip(city, ip.to_string()),
        Err(_) => {
            response.set_status(404);
            return Ok(response);
        }
    };
    let json = serde_json::to_string(&location)?;

    response.set_body(json);
    Ok(response)
}
