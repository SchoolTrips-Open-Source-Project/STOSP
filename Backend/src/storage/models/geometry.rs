use diesel::sql_types::Bool;
use geo::{coord, Coord};
use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize)]
pub struct  ServiceabilityRequest {
    pub point : Coord,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceabilityResponse {
    pub serviceable: bool,
}