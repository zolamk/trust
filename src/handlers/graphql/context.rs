use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    mailer::EmailTemplates,
    operator_signature::OperatorSignature,
    sms::SMSTemplates,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

pub struct Context {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
    pub config: Config,
    pub email_templates: EmailTemplates,
    pub sms_templates: SMSTemplates,
    pub operator_signature: OperatorSignature,
    pub token: Result<JWT, CryptoError>,
}

impl juniper::Context for Context {}

impl AsRef<Self> for Context {
    fn as_ref(&self) -> &Self {
        return self;
    }
}
