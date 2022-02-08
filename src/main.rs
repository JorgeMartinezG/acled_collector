#[macro_use]
extern crate diesel;

mod acled;
mod config;
mod db;
mod schema;

use config::Config;

use acled::AcledClient;

fn main() {
    let config = Config::new();
    println!("{:?}", config);

    let acled = AcledClient::new(&config.acled_params, config.countries.get("MOZ").unwrap());

    let response = acled.get_response(1);

    response
        .data
        .iter()
        .for_each(|res| println!("{}", res.iso3));

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
