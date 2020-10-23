use crate::{
    config::Config,
    crypto::{jwt::JWT, Error as CryptoError},
    operator_signature::OperatorSignature,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

pub struct Context {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
    pub config: Config,
    pub operator_signature: OperatorSignature,
    pub token: Result<JWT, CryptoError>,
}

impl juniper::Context for Context {}

impl AsRef<Self> for Context {
    fn as_ref(&self) -> &Self {
        return self;
    }
}
