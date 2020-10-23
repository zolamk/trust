use crate::config::Config;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use std::str::FromStr;

embed_migrations!("./migrations");

pub fn migrations() {
    let config = Config::new();

    let log_level = config.log_level.clone();

    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::from_str(&log_level).unwrap());

    let manager = ConnectionManager::<PgConnection>::new(config.database_url);

    let connection_pool = Pool::new(manager);

    if connection_pool.is_err() {
        println!("{:?}", connection_pool.err().unwrap());
        return;
    }

    let connection_pool = connection_pool.unwrap();

    let connection = connection_pool.get().expect("unable to get database connection");

    match embedded_migrations::run_with_output(&connection, &mut std::io::stdout()) {
        Ok(_val) => println!("migrations run successfully"),
        Err(val) => println!("{}", val),
    }
}
