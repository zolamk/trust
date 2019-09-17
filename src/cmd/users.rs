extern crate chrono;
extern crate clap;
extern crate diesel;

use clap::ArgMatches;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error::DatabaseError;

use crate::models::user::NewUser;

fn new_user(matches: Option<&ArgMatches>, connection_pool: Pool<ConnectionManager<PgConnection>>) {
    let matches = matches.unwrap();

    let mut user = NewUser::default();

    user.email = matches.value_of("email").unwrap().to_string();

    user.password = Some(matches.value_of("password").unwrap().to_string());

    user.aud = matches.value_of("aud").unwrap().to_string();

    user.name = match matches.value_of("name") {
        Some(n) => Some(n.to_string()),
        None => None,
    };

    user.role = match matches.is_present("admin") {
        true => Some("admin".to_string()),
        false => None,
    };

    user.is_super_admin = matches.is_present("super_admin");

    user.confirmed = matches.is_present("confirm");

    user.hash_password();

    let connection = connection_pool
        .get()
        .expect("unable to get connection to database");

    match user.save(&*connection) {
        Ok(_val) => println!("{} created successfully", user.email),
        Err(err) => match err {
            DatabaseError(_kind, _info) => println!("{} already exists!", user.email),
            _ => println!("{}", err),
        },
    }
}

fn delete_user(matches: Option<&ArgMatches>) {
    let matches = matches.unwrap();
}

pub fn users(matches: Option<&ArgMatches>, connection_pool: Pool<ConnectionManager<PgConnection>>) {
    let matches = matches.unwrap();

    match matches.subcommand() {
        ("create", sub_m) => new_user(sub_m, connection_pool),
        ("delete", sub_m) => delete_user(sub_m),
        _ => {}
    }
}
