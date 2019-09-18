#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod cmd;
mod config;
mod handlers;
mod models;
mod schema;

use clap::App;
use config::Config;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

fn run(config: Config) {
    let rocket_config = rocket::config::Config::build(rocket::config::Environment::Production)
        .address(config.host.clone())
        .port(config.port)
        .unwrap();

    let app = rocket::custom(rocket_config);

    app.mount("/", routes![handlers::health_check::health])
        .manage(config)
        .launch();
}

fn main() {
    let config = Config::new();

    let manager = ConnectionManager::<PgConnection>::new(config.database_url.clone());

    let pool = Pool::new(manager).unwrap();

    let cli_yaml = load_yaml!("cli.yml");

    let cli = App::from_yaml(cli_yaml);

    let matches = cli.get_matches();

    match matches.subcommand() {
        ("run", _sub_m) => run(config),
        ("users", sub_m) => cmd::users(sub_m, pool),
        ("migrate", sub_m) => cmd::migrations(pool),
        ("version", _sub_m) => cmd::version(),
        _ => {}
    }
}
