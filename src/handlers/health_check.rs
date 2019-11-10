extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket_contrib::json::Json;
use serde::Serialize;

use std::env;

#[derive(Serialize)]
pub struct HealthCheck {
    pub version: String,
    pub name: String,
    pub description: String,
}

#[get("/health")]
pub fn health() -> Json<HealthCheck> {
    return Json(HealthCheck {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: String::from("trust"),
        description: String::from("trust is a user authentication and authorization microservice written in rust"),
    });
}
