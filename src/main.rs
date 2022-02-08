#[macro_use]
extern crate diesel;

mod acled;
mod config;
mod db;
mod schema;

use config::Config;
use diesel::PgConnection;

use acled::AcledClient;

use diesel::prelude::*;
use log::info;

use diesel::debug_query;
use diesel::pg::Pg;
use schema::acled::incidents;

fn process_country<'a>(iso3: &'a str, config: &Config, db_url: &'a str) {
    info!("Processing country {iso3}");

    let mut page = 1;
    let mut total = 0;

    let conn = PgConnection::establish(db_url).expect("Error connecting to database");

    diesel::delete(incidents::table.filter(incidents::iso3.eq(iso3)))
        .execute(&conn)
        .expect("Could not delete rows");

    let stmt = diesel::delete(incidents::table.filter(incidents::iso3.eq(iso3)));
    println!("{:?}", debug_query::<Pg, _>(&stmt));

    let acled = AcledClient::new(&config.acled_params, *config.countries.get(iso3).unwrap());

    loop {
        let response = acled.get_response(page);
        diesel::insert_into(incidents::table)
            .values(response.data)
            .execute(&conn)
            .expect("Failed saving results");

        let count = response.count;

        if count == 0 {
            break;
        }

        page += 1;
        total += count;
        info!("{count}");
    }
    info!("Stored {total} total incidents");
}

fn main() {
    env_logger::init();

    let config = Config::new();

    let db_url: String = config.get_database_url();

    process_country("MOZ", &config, &db_url);

    /*


    let results = incidents::table
        .limit(5)
        .load::<Incident>(&conn)
        .expect("Error loading incidents");

    println!("{:?}", results);


    items.iter().for_each(|item| println!("{:?}", item));

    */
}
