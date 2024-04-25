use log::{error, info};
use std::{fs, str::FromStr};

use crate::tools::{contants::SHAPE_FILE_PATH, types::MultiPolygonWithName, utils::get_file_name_without_extension};

pub fn get_available_shape_files() -> Vec<MultiPolygonWithName> {
    let mut shape_files = Vec::new();
    match fs::read_dir(SHAPE_FILE_PATH) {
        Ok(entries) => {
            for entry in entries {
                let file_name = entry.unwrap().file_name().into_string().unwrap();
                match fs::read_to_string(SHAPE_FILE_PATH.to_owned() + "/" + &file_name) {
                    Ok(geo_json_str) => match geojson::Geometry::from_str(&geo_json_str) {
                        Ok(geojson) => {
                            let multi_polygon: geo::MultiPolygon =
                                TryFrom::try_from(geojson.to_owned()).unwrap();

                            shape_files.push(MultiPolygonWithName::new(get_file_name_without_extension(file_name), multi_polygon))
                        }
                        Err(err) => {
                            error!("Error while parsing geo json string {}", err);
                        }
                    },
                    Err(err) => {
                        error!("Error while reading geo json file {}", err);
                    }
                }
            }
            return shape_files;
        }
        Err(err) => {
            error!("Error while reading shape files dir {}", err);
            shape_files
        }
    }
}
