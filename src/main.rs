#![feature(proc_macro_hygiene, decl_macro)]
#![feature(type_ascription)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket_contrib;
extern crate lettre;
extern crate simple_logger;

mod cmd;
mod config;
mod crypto;
mod handlers;
mod mailer;
mod models;
mod schema;

use clap::App;
use config::Config;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use log::Level;
use mailer::EmailTemplates;
use std::str::FromStr;

fn run(connection_pool: Pool<ConnectionManager<PgConnection>>, config: Config) {
    let rocket_config = rocket::config::Config::build(rocket::config::Environment::Production)
        .address(config.host.clone())
        .port(config.port)
        .unwrap();

    let app = rocket::custom(rocket_config);

    let email_templates = EmailTemplates::new(config.clone());

    app.mount(
        "/",
        routes![
            handlers::health_check::health,
            handlers::users::signup::signup,
            handlers::users::confirm::confirm,
        ],
    )
    .manage(config)
    .manage(connection_pool)
    .manage(email_templates)
    .launch();
}

fn main() {
    let config = Config::new();

    let manager = ConnectionManager::<PgConnection>::new(config.database_url.clone());

    let pool = Pool::new(manager).unwrap();

    let cli_yaml = load_yaml!("cli.yml");

    let cli = App::from_yaml(cli_yaml);

    let matches = cli.get_matches();

    simple_logger::init_with_level(Level::from_str(&config.log_level).unwrap()).unwrap();

    match matches.subcommand() {
        ("run", _sub_m) => run(pool, Config::new()),
        ("users", sub_m) => cmd::users(sub_m, pool, Config::new()),
        ("migrate", _sub_m) => cmd::migrations(pool),
        ("version", _sub_m) => cmd::version(),
        _ => {}
    }
}
