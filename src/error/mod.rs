extern crate diesel;
extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::{Request, Response};
use serde::Serialize;
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct Error {
    pub code: u16,
    pub body: serde_json::Value,
}

impl Error {
    pub fn new(code: u16, body: serde_json::Value) -> Error {
        return Error { code, body };
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let body = serde_json::to_vec(&self.body).unwrap();

        let status = Status::from_code(self.code).unwrap();

        return Response::build()
            .sized_body(Cursor::new(body))
            .header(ContentType::JSON)
            .status(status)
            .ok();
    }
}

impl From<diesel::result::Error> for Error {
    fn from(_e: diesel::result::Error) -> Self {
        return Error {
            code: 500,
            body: json!({}),
        };
    }
}
