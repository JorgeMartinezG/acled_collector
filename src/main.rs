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

use schema::acled::incidents;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "AC/DC", about = "Acled client to postgres database")]
struct Opt {
    #[structopt(short = "c", long = "config", default_value = "config.toml")]
    config_file: PathBuf,
}

fn process_country<'a>(iso3: &'a str, config: &Config, db_url: &'a str) {
    info!("Processing country {iso3}");

    let mut page = 1;
    let mut total = 0;

    let conn = PgConnection::establish(db_url).expect("Error connecting to database");

    diesel::delete(incidents::table.filter(incidents::iso3.eq(iso3)))
        .execute(&conn)
        .expect("Could not delete rows");

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
    }
    info!("Stored {total} total incidents");
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();
    let config = Config::new(&opt.config_file);

    println!("{:?}", config);

    let db_url: String = config.get_database_url();

    config
        .countries
        .iter()
        .for_each(|(iso3, _code)| process_country(&iso3, &config, &db_url));
}
