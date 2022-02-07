#[macro_use]
extern crate diesel;

mod acled;
mod config;
mod db;
mod schema;

use config::Config;
use serde_json::Value;

use toml;

use acled::AcledRequest;

fn main() {
    let config = Config::new();
    println!("{:?}", config);

    let acled = AcledRequest::new(&config.acled_params);

    println!("{:?}", acled);

    /*

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
