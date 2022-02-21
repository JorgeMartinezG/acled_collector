pub mod incident;
mod request;
mod response;

use chrono::{Duration, NaiveDate, Utc};
use reqwest::blocking::Client;
use serde::Deserialize;

use incident::Incident;
use request::Request;
use response::Response;

use toml::Value;

#[derive(Debug)]
pub struct APIParams {
    api_url: String,
    key: String,
    email: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl APIParams {
    pub fn new(params: &Value, months: i64) -> APIParams {
        let end_date = Utc::today().naive_utc();
        let start_date = end_date - Duration::weeks(4 * months);

        APIParams {
            api_url: params
                .get("api_url")
                .expect("Missing api_url")
                .to_owned()
                .to_string(),
            key: params
                .get("key")
                .expect("Missing key")
                .to_owned()
                .to_string(),
            email: params
                .get("email")
                .expect("Missing email")
                .to_owned()
                .to_string(),
            start_date: start_date,
            end_date: end_date,
        }
    }
}

#[derive(Debug)]
pub struct AcledClient<'a> {
    client: Client,
    params: &'a APIParams,
    iso: u16,
}

impl<'a> AcledClient<'a> {
    pub fn new(params: &'a APIParams, iso: u16) -> Self {
        AcledClient {
            client: Client::new(),
            params: params,
            iso: iso,
        }
    }

    pub fn get_response(&self, page: u8) -> Response {
        let event_date = format!("{}|{}", self.params.start_date, self.params.end_date);
        let request_params = Request {
            key: &self.params.key,
            email: &self.params.email,
            page: page,
            iso: self.iso,
            event_date: &event_date,
            event_date_where: "BETWEEN",
        };

        self.client
            .get(&self.params.api_url)
            .query(&request_params)
            .send()
            .expect("Failed running request")
            .json()
            .expect("Failed parsing json")
    }
}
