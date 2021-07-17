#![allow(clippy::needless_return, clippy::module_inception, clippy::new_without_default, clippy::too_many_arguments)]
#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "256"]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate juniper;

pub mod cmd;
pub mod config;
pub mod crypto;
pub mod handlers;
pub mod hook;
pub mod mailer;
pub mod models;
mod schema;
pub mod sms;
