#[macro_use]
extern crate diesel;

mod acled;
mod config;
mod db;
mod schema;

use config::Config;

use acled::AcledClient;

use log::info;

fn process_country<'a>(iso3: &'a str, config: &Config) {
    info!("Processing country {iso3}");

    let acled = AcledClient::new(&config.acled_params, *config.countries.get(iso3).unwrap());

    let mut page = 1;
    let mut total = 0;

    loop {
        let response = acled.get_response(page);

        let count = response.count;

        if count > 0 {
            page += 1;
            total += count;
            info!("{count}");
        } else {
            break;
        }
    }
    info!("Stored {total} total incidents");
}

fn main() {
    env_logger::init();

    let config = Config::new();

    process_country("MOZ", &config);

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
