use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;

use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct AcledApiParams {
    api_url: String,
    key: String,
    email: String,
}

pub struct RequestParams<'a> {
    pub key: &'a str,
    pub email: &'a str,
    pub page: u8,
}

impl<'a> Serialize for RequestParams<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("RequestParams", 4)?;
        s.serialize_field("key", &self.key)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("page", &self.page)?;

        s.end()
    }
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

    pub fn get_acled_credentials(&self) -> (&str, &str, &str) {
        let params = &self.acled_params;

        return (&params.api_url, &params.key, &params.email);
    }
}
