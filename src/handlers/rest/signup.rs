use crate::{
    config::Config,
    handlers::{lib::signup, Error},
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use log::error;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

#[post("/signup", data = "<signup_form>")]
pub fn signup(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    signup_form: Json<signup::SignUpForm>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
) -> Result<JsonValue, Error> {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return Err(Error::from(err));
    }

    let internal_error = Error {
        code: 500,
        message: "Internal Server Error".to_string(),
        body: json!({
            "code": "internal_error"
        }),
    };

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return Err(internal_error);
        }
    };

    let user = signup::signup(config.inner(), &connection, signup_form.into_inner());

    if user.is_err() {
        return Err(user.err().unwrap());
    }

    let body = json!({
        "code": "success",
        "message": "user has been successfully signed up"
    });

    return Ok(JsonValue(body));
}
