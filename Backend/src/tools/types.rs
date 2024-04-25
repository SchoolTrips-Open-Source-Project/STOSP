use geo::MultiPolygon;
use serde::{Deserialize, Serialize};

#[derive (Serialize, Deserialize)]
pub struct MultiPolygonWithName {
    name : String,
    polygon : MultiPolygon,
}

impl MultiPolygonWithName {
    pub fn new(name:String, polygon: MultiPolygon) -> Self {
        MultiPolygonWithName{
            name
        ,   polygon
        }        
    }

    pub fn get_name(self) -> String {
        return  self.name;
    }

    pub fn get_polygon(self) -> MultiPolygon {
        return  self.polygon;
    }
}