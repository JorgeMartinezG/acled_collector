mod incident;
mod request;
mod response;

use chrono::{Duration, NaiveDate, Utc};
use reqwest::blocking::Client;
use serde::Deserialize;

use incident::Incident;
use request::Request;
use response::Response;

#[derive(Deserialize, Debug)]
pub struct APIParams {
    api_url: String,
    key: String,
    email: String,
}

#[derive(Debug)]
pub struct AcledClient<'a> {
    client: Client,
    params: &'a APIParams,
    start_date: NaiveDate,
    end_date: NaiveDate,
    iso: u16,
}

impl<'a> AcledClient<'a> {
    pub fn new(params: &'a APIParams, iso: u16) -> Self {
        let end_date = Utc::today().naive_utc();
        let start_date = end_date - Duration::days(365 * 3);

        AcledClient {
            client: Client::new(),
            params: params,
            iso: iso,
            start_date: start_date,
            end_date: end_date,
        }
    }

    pub fn get_response(&self, page: u8) -> Response {
        let event_date = format!("{}|{}", self.start_date, self.end_date);
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