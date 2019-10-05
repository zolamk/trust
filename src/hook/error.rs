extern crate reqwest;
extern crate rocket;
extern crate serde_json;

use rocket::http::Status;
use serde_json::Value;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub status: Status,
    pub body: Value,
}

impl Error {
    pub fn new(status: Status, body: Value) -> Error {
        return Error {
            status: status,
            body: body,
        };
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{} error in hook", self.status);
    }
}
