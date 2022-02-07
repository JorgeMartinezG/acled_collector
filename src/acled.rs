use crate::db::PointType;
use chrono::NaiveDate;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize, Deserializer};

use postgis::ewkb::Point;

use serde_json::Value;

use crate::schema::acled::incidents;

#[derive(Queryable, Debug, Insertable)]
#[table_name = "incidents"]
struct Incident {
    data_id: i64,
    iso: i64,
    event_id_cnty: String,
    event_id_no_cnty: i64,
    event_date: NaiveDate,
    year: i64,
    time_precision: i64,
    event_type: String,
    sub_event_type: String,
    actor1: String,
    assoc_actor_1: String,
    inter1: i64,
    actor2: String,
    assoc_actor_2: String,
    inter2: i64,
    interaction: String,
    region: String,
    country: String,
    admin1: String,
    admin2: String,
    admin3: String,
    location: String,
    geo_precision: i64,
    source: String,
    source_scale: String,
    notes: String,
    fatalities: i64,
    timestamp: i64,
    iso3: String,
    geom: PointType,
}

#[derive(Deserialize, Debug)]
pub struct APIParams {
    api_url: String,
    key: String,
    email: String,
}

pub struct APIRequest<'a> {
    pub key: &'a str,
    pub email: &'a str,
    pub page: u8,
}

impl<'a> Serialize for APIRequest<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("APIRequest", 4)?;
        s.serialize_field("key", &self.key)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("page", &self.page)?;

        s.end()
    }
}

impl APIParams {
    pub fn get_acled_credentials(&self) -> (&str, &str, &str) {
        return (&self.api_url, &self.key, &self.email);
    }
}

impl<'de> Deserialize<'de> for Incident {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: Value = Value::deserialize(deserializer)?;

        let latitude = json
            .get("latitude")
            .expect("latitude not found")
            .as_str()
            .expect("latitude is not string")
            .parse::<f64>()
            .expect("Failed parsing latitude");

        let longitude = json
            .get("longitude")
            .expect("longitude not found")
            .as_str()
            .expect("longitude is not string")
            .parse::<f64>()
            .expect("Failed parsing longitude");

        Ok(Self {
            data_id: json
                .get("data_id")
                .expect("data_id not found")
                .as_str()
                .expect("data_id is not string")
                .parse::<i64>()
                .expect("Failed parsing data_id"),
            iso: json
                .get("iso")
                .expect("iso not found")
                .as_str()
                .expect("iso is not string")
                .parse::<i64>()
                .expect("Failed parsing iso"),
            event_id_cnty: json
                .get("event_id_cnty")
                .expect("event_id_cnty")
                .to_string(),
            event_id_no_cnty: json
                .get("event_id_no_cnty")
                .expect("event_id_no_cnty not found")
                .as_str()
                .expect("event_id_no_cnty is not string")
                .parse::<i64>()
                .expect("Failed parsing event_id_no_cnty"),
            event_date: NaiveDate::parse_from_str(
                json.get("event_date")
                    .expect("event_date not found")
                    .as_str()
                    .expect("event_date is not a string"),
                "%Y-%m-%d",
            )
            .expect("Failed parsing event_date"),
            year: json
                .get("year")
                .expect("year not found")
                .as_str()
                .expect("year is not string")
                .parse::<i64>()
                .expect("Failed parsing year"),
            time_precision: json
                .get("time_precision")
                .expect("time_precision not found")
                .as_str()
                .expect("time_precision is not string")
                .parse::<i64>()
                .expect("Failed parsing time_precision"),
            event_type: json.get("event_type").expect("event_type").to_string(),
            sub_event_type: json
                .get("sub_event_type")
                .expect("sub_event_type")
                .to_string(),
            actor1: json.get("actor1").expect("actor1").to_string(),
            assoc_actor_1: json
                .get("assoc_actor_1")
                .expect("assoc_actor_1")
                .to_string(),
            inter1: json
                .get("inter1")
                .expect("inter1 not found")
                .as_str()
                .expect("inter1 is not string")
                .parse::<i64>()
                .expect("Failed parsing inter1"),
            actor2: json.get("actor2").expect("actor2").to_string(),
            assoc_actor_2: json
                .get("assoc_actor_2")
                .expect("assoc_actor_2")
                .to_string(),
            inter2: json
                .get("inter2")
                .expect("inter2 not found")
                .as_str()
                .expect("inter2 is not string")
                .parse::<i64>()
                .expect("Failed parsing inter2"),
            interaction: json.get("interaction").expect("interaction").to_string(),
            region: json.get("region").expect("region").to_string(),
            country: json.get("country").expect("country").to_string(),
            admin1: json.get("admin1").expect("admin1").to_string(),
            admin2: json.get("admin2").expect("admin2").to_string(),
            admin3: json.get("admin3").expect("admin3").to_string(),
            location: json.get("location").expect("location").to_string(),
            geo_precision: json
                .get("geo_precision")
                .expect("geo_precision not found")
                .as_str()
                .expect("geo_precision is not string")
                .parse::<i64>()
                .expect("Failed parsing geo_precision"),
            source: json.get("source").expect("source").to_string(),
            source_scale: json.get("source_scale").expect("source_scale").to_string(),
            notes: json.get("notes").expect("notes").to_string(),
            fatalities: json
                .get("fatalities")
                .expect("fatalities not found")
                .as_str()
                .expect("fatalities is not string")
                .parse::<i64>()
                .expect("Failed parsing fatalities"),
            timestamp: json
                .get("timestamp")
                .expect("timestamp not found")
                .as_str()
                .expect("timestamp is not string")
                .parse::<i64>()
                .expect("Failed parsing timestamp"),
            iso3: json.get("iso3").expect("iso3").to_string(),
            geom: PointType(Point::new(longitude, latitude, Some(4326))),
        })
    }
}

#[derive(Deserialize, Debug)]
struct Response {
    status: u8,
    success: bool,
    last_update: i32,
    count: u32,
    data: Vec<Incident>,
    filename: String,
}
