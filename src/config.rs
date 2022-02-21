use std::collections::HashMap;

use crate::acled::APIParams;
use crate::db::Database;

use serde::{Deserialize, Deserializer};

use std::fs::read_to_string;
use std::path::PathBuf;

use toml::Value;

use chrono::{Duration, NaiveDate, Utc};

#[derive(Debug)]
pub struct Config {
    pub database: Database,
    pub acled_params: APIParams,
    pub countries: HashMap<String, u16>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Config {
    pub fn new(config_file: &PathBuf) -> Self {
        let content = read_to_string(config_file).unwrap();

        let config: Config = toml::from_str(content.as_str()).expect("Cannot read config file");

        config
    }

    pub fn get_database_url(&self) -> String {
        self.database.get_url()
    }
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let database: Database = Value::try_into(
            value
                .get("database")
                .expect("could not get database field")
                .to_owned(),
        )
        .expect("could not deserialize database");

        let acled_params: APIParams = Value::try_into(
            value
                .get("acled_params")
                .expect("could not get acled_params field")
                .to_owned(),
        )
        .expect("could not deserialize acled_params");

        let countries: HashMap<String, u16> = Value::try_into(
            value
                .get("countries")
                .expect("could not get countries field")
                .to_owned(),
        )
        .expect("could not deserialize countries");

        let end_date = Utc::today().naive_utc();

        let months = value
            .get("months")
            .expect("months not found")
            .as_integer()
            .expect("Failed parsing months");

        let start_date = end_date - Duration::weeks(4 * months);

        let config = Config {
            database: database,
            acled_params: acled_params,
            countries: countries,
            start_date: start_date,
            end_date: end_date,
        };

        Ok(config)
    }
}
