use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ServiceabilityRequest {
    pub point: LatLong,
}

#[derive(Deserialize, Serialize)]
pub struct LatLong {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceabilityResponse {
    pub serviceable: bool,
}
