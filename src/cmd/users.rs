use crate::{
    config::Config,
    models::{
        user::{NewUser, User},
        Error,
    },
};
use clap::ArgMatches;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;

fn new_user(matches: Option<&ArgMatches>, connection_pool: Pool<ConnectionManager<PgConnection>>, config: Config) {
    let matches = matches.unwrap();

    let mut user = NewUser::default();

    user.email = matches.value_of("email").unwrap().to_string();

    user.password = Some(matches.value_of("password").unwrap().to_string());

    user.aud = matches.value_of("aud").unwrap().to_string();

    user.is_admin = matches.is_present("admin");

    user.confirmed = config.auto_confirm || matches.is_present("confirm");

    user.hash_password();

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(err) => {
            error!("unable to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };

    match user.save(&connection) {
        Ok(_val) => println!("{} created successfully", user.email),
        Err(err) => match err {
            Error::DatabaseError(_) => println!("{} already exists!", user.email),
            _ => println!("{:?}", err),
        },
    }
}

fn remove_user(matches: Option<&ArgMatches>, connection_pool: Pool<ConnectionManager<PgConnection>>) {
    let connection = connection_pool.get().expect("unable to get connection to database");

    let matches = matches.unwrap();

    let email = matches.value_of("email").unwrap().to_string();

    match User::delete_by_email(email, &connection) {
        Ok(_val) => println!("user deleted successfully"),
        Err(err) => println!("unable to delte user: {:?}", err),
    }
}

pub fn users(matches: Option<&ArgMatches>, connection_pool: Pool<ConnectionManager<PgConnection>>, config: Config) {
    let matches = matches.unwrap();

    match matches.subcommand() {
        ("create", sub_m) => new_user(sub_m, connection_pool, config),
        ("remove", sub_m) => remove_user(sub_m, connection_pool),
        _ => {}
    }
}
