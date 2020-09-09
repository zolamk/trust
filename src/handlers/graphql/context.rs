use crate::{config::Config, mailer::EmailTemplates, operator_signature::OperatorSignature};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

pub struct Context {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
    pub config: Config,
    pub email_templates: EmailTemplates,
    pub operator_signature: OperatorSignature,
}

impl juniper::Context for Context {}

impl AsRef<Self> for Context {
    fn as_ref(&self) -> &Self {
        return self;
    }
}
