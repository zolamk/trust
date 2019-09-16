#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod config;
mod handlers;

use config::Config;

fn main() {
    let c = Config::new();

    let rc = rocket::config::Config::build(rocket::config::Environment::Production)
        .address(c.host.clone())
        .port(c.port)
        .unwrap();

    let app = rocket::custom(rc);

    app.mount("/", routes![handlers::health_check::health])
        .manage(c)
        .launch();
}
