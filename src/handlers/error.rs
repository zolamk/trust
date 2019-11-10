use crate::crypto::Error as CryptoError;
use crate::hook::Error as HookError;
use crate::mailer::Error as MailerError;
use crate::models::Error as ModelError;
use crate::operator_signature::Error as OperatorSignatureError;
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

impl From<HookError> for Error {
    fn from(e: HookError) -> Self {
        if let HookError::HookError(err) = e {
            return Error::new(err.code, err.body);
        }

        if let HookError::RequestError(_) = e {
            return Error::new(
                422,
                json!({
                    "code": "webhook_error"
                }),
            );
        }

        if let HookError::JSONError(_) = e {
            return Error::new(
                422,
                json!({
                    "code": "unprocessable_webhook_response"
                }),
            );
        }

        return Error::new(
            500,
            json!({
                "code": "internal_error"
            }),
        );
    }
}

impl From<ModelError> for Error {
    fn from(_: ModelError) -> Self {
        return Error::new(
            500,
            json!({
                "code": "internal_error"
            }),
        );
    }
}

impl From<OperatorSignatureError> for Error {
    fn from(e: OperatorSignatureError) -> Self {
        if let OperatorSignatureError::SignatureMissing = e {
            return Error::new(
                400,
                json!({
                    "code": "operator_signature_missing"
                }),
            );
        }

        if let OperatorSignatureError::JSONError(_) = e {
            return Error::new(
                500,
                json!({
                    "code": "operator_signature_json_error"
                }),
            );
        }

        if let OperatorSignatureError::JWTError(_) = e {
            return Error::new(
                400,
                json!({
                    "code": "invalid_operator_signature"
                }),
            );
        }

        return Error::new(
            500,
            json!({
                "code": "internal_error"
            }),
        );
    }
}

impl From<MailerError> for Error {
    fn from(e: MailerError) -> Self {
        if let MailerError::TemplateError(_) = e {
            return Error::new(
                500,
                json!({
                    "code": "email_template_error"
                }),
            );
        }

        return Error::new(
            500,
            json!({
                "code": "email_internal_error"
            }),
        );
    }
}

impl From<CryptoError> for Error {
    fn from(e: CryptoError) -> Self {
        if let CryptoError::TokenMissing = e {
            return Error::new(
                401,
                json!({
                    "code": "access_token_missing"
                }),
            );
        }

        if let CryptoError::JWTError(_) = e {
            return Error::new(
                401,
                json!({
                    "code": "invalid_access_token"
                }),
            );
        }

        return Error::new(
            500,
            json!({
                "code": "access_token_internal_error"
            }),
        );
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        return Error {
            code: 500,
            body: json!({
                "code": "database_error"
            }),
        };
    }
}
