#![allow(clippy::needless_return, clippy::module_inception)]
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
extern crate serde_json;
extern crate lettre;
extern crate simple_logger;

mod cmd;
mod config;
mod crypto;
mod handlers;
mod hook;
mod mailer;
mod models;
mod operator_signature;
mod schema;

use clap::App;
use config::Config;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use log::{info, Level};
use mailer::EmailTemplates;
use std::str::FromStr;

fn run(connection_pool: Pool<ConnectionManager<PgConnection>>, config: Config, email_templates: EmailTemplates) {
    let host = config.host.clone();

    let port = config.port;

    let rocket_config = rocket::config::Config::build(rocket::config::Environment::Production).address(host.clone()).port(port).unwrap();

    let app = rocket::custom(rocket_config);

    info!("trust running on {:?}:{:?}", host, port);

    app.mount(
        "/",
        routes![
            handlers::health_check::health,
            handlers::users::signup::signup,
            handlers::users::confirm::confirm,
            handlers::users::token::token,
            handlers::users::invite::invite,
            handlers::users::authorize::authorize,
            handlers::users::callback::callback,
            handlers::users::user::get::get,
            handlers::users::users::create::create,
            handlers::users::users::delete::delete,
            handlers::users::user::change_password::change_password,
            handlers::users::user::change_email::change_email,
            handlers::users::user::change_email_confirm::change_email_confirm,
        ],
    )
    .manage(config)
    .manage(connection_pool)
    .manage(email_templates)
    .launch();
}

fn main() {
    let config = Config::new();

    let database_url = config.database_url.clone();

    let log_level = config.log_level.clone();

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::new(manager).unwrap();

    let email_templates = EmailTemplates::new(config.clone());

    let cli_yaml = load_yaml!("cli.yml");

    let cli = App::from_yaml(cli_yaml);

    let matches = cli.get_matches();

    simple_logger::init_with_level(Level::from_str(&log_level).unwrap()).unwrap();

    match matches.subcommand() {
        ("run", _sub_m) => run(pool, config, email_templates),
        ("users", sub_m) => cmd::users(sub_m, pool, config, email_templates),
        ("operator", sub_m) => cmd::operator(sub_m, config),
        ("migrate", _sub_m) => cmd::migrations(pool),
        ("version", _sub_m) => cmd::version(),
        _ => {}
    }
}
