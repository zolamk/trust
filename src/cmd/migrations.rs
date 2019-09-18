use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

embed_migrations!("./migrations");

pub fn migrations(connection_pool: Pool<ConnectionManager<PgConnection>>) {
    let connection = connection_pool
        .get()
        .expect("unable to get database connection");

    match embedded_migrations::run_with_output(&connection, &mut std::io::stdout()) {
        Ok(_val) => println!("migrations run successfully"),
        Err(val) => println!("{}", val),
    }
}
