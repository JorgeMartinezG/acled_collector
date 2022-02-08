use serde::Deserialize;

use std::collections::HashMap;

use crate::acled::APIParams;
use crate::db::Database;

use std::fs::read_to_string;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: Database,
    pub acled_params: APIParams,
    pub countries: HashMap<String, u16>,
}

impl Config {
    pub fn new() -> Self {
        let content = read_to_string("config.toml").unwrap();

        let config: Config = toml::from_str(content.as_str()).expect("Cannot read config file");

        config
    }

    pub fn get_database_url(&self) -> String {
        self.database.get_url()
    }
}
