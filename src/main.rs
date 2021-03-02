#![allow(clippy::needless_return, clippy::module_inception, clippy::new_without_default, clippy::too_many_arguments)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(type_ascription)]
#![recursion_limit = "256"]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate juniper;
extern crate lettre;
extern crate simple_logger;

mod cmd;
mod config;
mod cors;
mod crypto;
mod handlers;
mod hook;
mod mailer;
mod models;
mod operator_signature;
mod schema;
mod sms;

use crate::handlers::graphql::create_schema;
use clap::App;
use config::Config;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use log::info;
use std::str::FromStr;

fn run() {
    let config = Config::new();

    let database_url = config.database_url.clone();

    let log_level = config.log_level.clone();

    let logger = simple_logger::SimpleLogger::new().with_level(log::LevelFilter::from_str(&log_level).unwrap()).init();

    if logger.is_err() {
        let err = logger.err().unwrap();
        panic!("{}", err);
    }

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let connection_pool = Pool::builder().max_size(config.max_connection_pool_size).build(manager).unwrap();

    let host = config.host.clone();

    let port = config.port;

    let rocket_config = rocket::config::Config::build(rocket::config::Environment::Production).address(host.clone()).port(port).unwrap();

    let app = rocket::custom(rocket_config);

    info!("trust running on {:?}:{:?}", host, port);

    app.mount("/", routes![handlers::health_check::health, handlers::graphql::graphiql, handlers::graphql::graphql,])
        .manage(config)
        .manage(connection_pool)
        .manage(create_schema())
        .attach(cors::CORS)
        .launch();
}

fn main() {
    dotenv().ok();

    let cli_yaml = load_yaml!("cli.yml");

    let cli = App::from_yaml(cli_yaml);

    let matches = cli.get_matches();

    match matches.subcommand() {
        ("run", _sub_m) => run(),
        ("users", sub_m) => cmd::users(sub_m),
        ("operator", sub_m) => cmd::operator(sub_m),
        ("migrate", _sub_m) => cmd::migrations(),
        ("version", _sub_m) => cmd::version(),
        _ => {}
    }
}
