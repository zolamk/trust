use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

use crate::config::Config;

embed_migrations!("./migrations");

pub fn migrations() {
    let config = Config::new();

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
