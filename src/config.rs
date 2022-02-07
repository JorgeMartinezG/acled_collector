use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct AcledApiParams {
    api_url: String,
    key: String,
    email: String,
}

#[derive(Deserialize, Debug)]
struct Database {
    host: String,
    user: String,
    pw: String,
    port: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    database: Database,
    acled_params: AcledApiParams,
    countries: HashMap<String, String>,
}

impl Config {
    pub fn get_database_url(&self) -> String {
        let db = &self.database;

        format!(
            "postgis://{}:{}@{}:{}/{}",
            db.user, db.pw, db.host, db.port, db.name
        )
    }
}
