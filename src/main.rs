#[macro_use]
extern crate diesel;

pub mod config;
pub mod schema;
pub mod sql_types;

use self::schema::acled::incidents;
use chrono::NaiveDate;
use config::{Config, RequestParams};

use diesel::prelude::*;
use reqwest::blocking::Client;

use serde_json::Value;

use serde::{Deserialize, Deserializer};

use postgis::ewkb::Point;
use std::fs::read_to_string;
use toml;

use std::io::Cursor;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;

use diesel::serialize::{self, IsNull, Output, ToSql};
use postgis::ewkb::{AsEwkbPoint, EwkbRead, EwkbWrite, GeometryT};

use sql_types::Geometry;

#[derive(Debug, AsExpression, FromSqlRow)]
#[sql_type = "Geometry"]
struct PointType(Point);

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

impl FromSql<Geometry, Pg> for PointType {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        let mut r = Cursor::new(bytes);
        let geom = GeometryT::read_ewkb(&mut r)?;
        return match geom {
            postgis::ewkb::GeometryT::Point(point) => Ok(PointType(point)),
            _ => Err("Geometry is not a point".into()),
        };
    }
}

impl<Db: Backend> ToSql<Geometry, Db> for PointType {
    fn to_sql<W: std::io::Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        self.0.as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
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

fn get_config() -> Config {
    let content = read_to_string("config.toml").unwrap();

    let config: Config = toml::from_str(content.as_str()).expect("Cannot read config file");

    config
}

fn main() {
    let config = get_config();

    //let db_string: String = config.get_database_url();
    let (url, key, email) = config.get_acled_credentials();

    let request_params = RequestParams {
        key: key,
        email: email,
        page: 1,
    };

    let client = Client::new();
    let res = client.get(url).query(&request_params);

    println!("{:?}", res);

    /*

    let conn = PgConnection::establish("postgres://postgres:postgres@localhost:5433/postgres")
        .expect("Error connecting to database");

    let results = incidents::table
        .limit(5)
        .load::<Incident>(&conn)
        .expect("Error loading incidents");

    println!("{:?}", results);

    let items: Vec<Incident> = diesel::insert_into(incidents::table)
        .values(res.data)
        .get_results(&conn)
        .expect("Failed saving results");

    items.iter().for_each(|item| println!("{:?}", item));

    */
}
