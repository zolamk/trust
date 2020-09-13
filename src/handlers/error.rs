use crate::{crypto::Error as CryptoError, hook::Error as HookError, mailer::Error as MailerError, models::Error as ModelError, operator_signature::Error as OperatorSignatureError};
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde::Serialize;
use serde_json::Error as SerdeError;
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct Error {
    pub code: u16,
    pub body: serde_json::Value,
    pub message: String,
}

impl Error {
    pub fn new(code: u16, body: serde_json::Value, message: String) -> Error {
        return Error { code, body, message };
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let body = serde_json::to_vec(&self.body).unwrap();

        let status = Status::from_code(self.code).unwrap();

        return Response::build().sized_body(Cursor::new(body)).header(ContentType::JSON).status(status).ok();
    }
}

impl From<SerdeError> for Error {
    fn from(_e: SerdeError) -> Self {
        return Error::new(
            500,
            json!({
                "code": "json_error"
            }),
            "Internal Server Error".to_string(),
        );
    }
}

impl From<HookError> for Error {
    fn from(e: HookError) -> Self {
        if let HookError::HookError(err) = e {
            return Error::new(err.code, err.body, "Webhook error".to_string());
        }

        if let HookError::RequestError(_) = e {
            return Error::new(
                422,
                json!({
                    "code": "webhook_error"
                }),
                "Webhook request error".to_string(),
            );
        }

        if let HookError::JSONError(_) = e {
            return Error::new(
                422,
                json!({
                    "code": "unprocessable_webhook_response"
                }),
                "Unprocessable webhook response".to_string(),
            );
        }

        return Error::new(
            500,
            json!({
                "code": "internal_error"
            }),
            "Internal server error while processing webhook".to_string(),
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
            "Internal Server Error".to_string(),
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
                "Operator Signature Missing".to_string(),
            );
        }

        if let OperatorSignatureError::JSONError(_) = e {
            return Error::new(
                500,
                json!({
                    "code": "operator_signature_json_error"
                }),
                "Operator Signature JSON Error".to_string(),
            );
        }

        if let OperatorSignatureError::JWTError(_) = e {
            return Error::new(
                400,
                json!({
                    "code": "invalid_operator_signature"
                }),
                "Invalid Operator Signature".to_string(),
            );
        }

        return Error::new(
            500,
            json!({
                "code": "internal_error"
            }),
            "Internal Server Error".to_string(),
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
                "Email Template Error".to_string(),
            );
        }

        return Error::new(
            500,
            json!({
                "code": "email_internal_error"
            }),
            "Internal Server Error While Processing Email".to_string(),
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
                "Access Token Missing".to_string(),
            );
        }

        if let CryptoError::JWTError(_) = e {
            return Error::new(
                401,
                json!({
                    "code": "invalid_access_token"
                }),
                "Invalid Access Token".to_string(),
            );
        }

        return Error::new(
            500,
            json!({
                "code": "access_token_internal_error"
            }),
            "Internal Server Error While Processing Access Token".to_string(),
        );
    }
}

impl From<&CryptoError> for Error {
    fn from(e: &CryptoError) -> Self {
        if let CryptoError::TokenMissing = e {
            return Error::new(
                401,
                json!({
                    "code": "access_token_missing"
                }),
                "Access Token Missing".to_string(),
            );
        }

        if let CryptoError::JWTError(_) = e {
            return Error::new(
                401,
                json!({
                    "code": "invalid_access_token"
                }),
                "Invalid Access Token".to_string(),
            );
        }

        return Error::new(
            500,
            json!({
                "code": "access_token_internal_error"
            }),
            "Internal Server Error While Processing Access Token".to_string(),
        );
    }
}

impl From<diesel::result::Error> for Error {
    fn from(_: diesel::result::Error) -> Self {
        return Error::new(
            500,
            json!({
                "code": "database_error"
            }),
            "Database Error".to_string(),
        );
    }
}

impl juniper::IntoFieldError for Error {
    fn into_field_error(self) -> juniper::FieldError {
        // TODO: pass the error body to graphql
        return juniper::FieldError::new(self.message, juniper::Value::null());
    }
}
