use serde::Deserialize;

use crate::db::{self, database::Database};

/// Servers state to handle the API
pub struct ServerState {
    pub data: Database,
    pub config: ServerConfig,
}

impl ServerState {
    pub fn new() -> Self {
        ServerState {
            data: db::database::Database::new(), // creating new db pool
            config: ServerConfig::new(),         // reading and creating config
        }
    }
}

#[derive(Deserialize)] // To deserialize from dhall config file
pub struct ServerConfig {
    pub default_otp: String,
    pub auth_timeout: i64,
    pub jwt_secret: String,
}

impl ServerConfig {
    pub fn new() -> Self {
        serde_dhall::from_file("./configs/app.dhall")
            .parse() // Reads the dhall config and tries to parse.
            .unwrap()
    }
}
