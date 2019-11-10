use crate::config::Config;
use crate::crypto::jwt::JWT;
use crate::crypto::secure_token;
use crate::handlers::trigger_hook;
use crate::handlers::users::provider::{FacebookProvider, Provider, ProviderState};
use crate::hook::HookEvent;
use crate::mailer::send_confirmation_email;
use crate::mailer::EmailTemplates;
use crate::models::refresh_token::NewRefreshToken;
use crate::models::user::{get_by_email, NewUser};
use crate::models::Error as ModelError;
use crate::operator_signature::{Error as OperatorSignatureError, OperatorSignature};
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::{DatabaseError, NotFound};
use diesel::Connection;
use log::error;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};
use rocket::response::Redirect;
use rocket::State;
use url::Url;

#[get("/authorize/callback?<code>&<state>")]
pub fn callback(
    config: State<Config>,
    connection_pool: State<Pool<ConnectionManager<PgConnection>>>,
    operator_signature: Result<OperatorSignature, OperatorSignatureError>,
    email_templates: State<EmailTemplates>,
    code: String,
    state: String,
) -> Redirect {
    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        let redirect_url = format!("{}?error=invalid_operator_signature", config.site_url);

        return Redirect::to(redirect_url);
    }

    let operator_signature = operator_signature.unwrap();

    let state = ProviderState::verify(state, config.inner());

    if state.is_err() {
        let err = state.err().unwrap();

        error!("{:?}", err);

        let redirect_url = format!("{}?error=invalid_oauth_state", operator_signature.site_url);

        return Redirect::to(redirect_url);
    }

    let state = state.unwrap();

    let provider_disabled = Redirect::to(format!(
        "{}?error=provider_disabled",
        operator_signature.site_url
    ));

    let internal_error = Redirect::to(format!(
        "{}?error=internal_error",
        operator_signature.site_url
    ));

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return internal_error;
        }
    };

    let oauth_provider = match state.provider.as_str() {
        "facebook" => {
            if !config.facebook_enabled {
                return provider_disabled;
            }
            Box::new(FacebookProvider::new(config.inner().clone()))
        }
        _ => {
            let redirect_url = format!("{}?error=invalid_provider", config.site_url);

            return Redirect::to(redirect_url);
        }
    };

    let client_id = oauth_provider.clone().client_id();

    let client_id = ClientId::new(client_id);

    let client_secret = Some(ClientSecret::new(
        config.facebook_client_secret.clone().unwrap(),
    ));

    let auth_url = oauth_provider.clone().auth_url();

    let auth_url = Url::parse(auth_url.as_str());

    if auth_url.is_err() {
        let err = auth_url.err().unwrap();

        error!("{:?}", err);

        return internal_error;
    }

    let auth_url = AuthUrl::new(auth_url.unwrap());

    let token_url = oauth_provider.clone().token_url();

    let token_url = Url::parse(token_url.as_str());

    if token_url.is_err() {
        let err = token_url.err().unwrap();

        error!("{:?}", err);

        return internal_error;
    }

    let token_url = Some(TokenUrl::new(token_url.unwrap()));

    let redirect_url = format!("{}/authorize/callback", config.instance_url);

    let redirect_url = Url::parse(redirect_url.as_str());

    if redirect_url.is_err() {
        let err = redirect_url.err().unwrap();

        error!("{:?}", err);

        return internal_error;
    }

    let redirect_url = RedirectUrl::new(redirect_url.unwrap());

    let client = BasicClient::new(client_id, client_secret, auth_url, token_url)
        .set_redirect_url(redirect_url);

    let access_token = client
        .exchange_code(AuthorizationCode::new(code))
        .request(http_client);

    if access_token.is_err() {
        let err = access_token.err().unwrap();

        error!("{:?}", err);

        let redirect_url = format!("{}?error=error_exchanging_code", config.site_url);

        return Redirect::to(redirect_url);
    }

    let access_token = access_token.unwrap();

    let access_token = access_token.access_token().secret();

    let user_data = oauth_provider.get_user_data(access_token.clone());

    if user_data.is_err() {
        let err = user_data.err().unwrap();

        error!("{:?}", err);

        let redirect_url = format!("{}?error=error_getting_user_data", config.site_url);

        return Redirect::to(redirect_url);
    }

    let user_data = user_data.unwrap();

    if user_data.email.is_none() {
        let redirect_url = format!(
            "{}?error=unable_to_find_email_with_provider",
            config.site_url
        );

        return Redirect::to(redirect_url);
    }

    let email = user_data.email.clone().unwrap();

    let transaction = connection.transaction::<Redirect, CallbackError, _>(|| {
        let mut user = get_by_email(email.clone(), &connection);

        let internal_error = CallbackError::new(format!(
            "{}?error=internal_error",
            operator_signature.site_url
        ));

        // if there was error getting use by email
        if user.is_err() {
            let err = user.err().unwrap();

            // if the user doesn't exist create the user
            // and trigger signup hook
            match err {
                ModelError::DatabaseError(NotFound) => {
                    if config.disable_signup {
                        let redirect_url = format!("{}?error=signup_disabled", config.site_url);

                        return Err(CallbackError::new(redirect_url));
                    }

                    let new_user = NewUser {
                        email,
                        aud: config.aud.clone(),
                        confirmed: true,
                        user_metadata: user_data.metadata,
                        confirmation_sent_at: None,
                        confirmation_token: None,
                        invitation_sent_at: None,
                        is_admin: false,
                        password: None,
                    };

                    user = new_user.save(&connection);

                    if user.is_err() {
                        let err = user.err().unwrap();

                        match err {
                            ModelError::DatabaseError(DatabaseError(
                                DatabaseErrorKind::UniqueViolation,
                                _info,
                            )) => {
                                let redirect_url = format!(
                                    "{}?error=email_already_registered",
                                    operator_signature.site_url
                                );
                                return Err(CallbackError::new(redirect_url));
                            }
                            err => {
                                error!("{:?}", err);

                                return Err(internal_error);
                            }
                        }
                    }

                    let user = trigger_hook(
                        HookEvent::Signup,
                        user.unwrap(),
                        config.inner(),
                        &connection,
                        operator_signature,
                        state.provider,
                    );

                    if user.is_err() {}

                    let user = user.unwrap();

                    let jwt = JWT::new(&user, config.aud.clone());

                    let jwt = jwt.sign(config.inner());

                    if jwt.is_err() {
                        let err = jwt.err().unwrap();

                        error!("{:?}", err);

                        let redirect_url =
                            format!("{}?error=unable_to_create_access_token", config.site_url);

                        return Err(CallbackError::new(redirect_url));
                    }

                    let jwt = jwt.unwrap();

                    let refresh_token = NewRefreshToken::new(user.id);

                    let refresh_token = refresh_token.save(&connection);

                    if refresh_token.is_err() {
                        let err = refresh_token.err();

                        error!("{:?}", err);

                        let redirect_url =
                            format!("{}?error=unable_to_create_refresh_token", config.site_url);

                        return Err(CallbackError::new(redirect_url));
                    }

                    let refresh_token = refresh_token.unwrap().token;

                    let redirect_url = format!(
                        "{}?access_token={}&type={}&refresh_token={}",
                        config.site_url, jwt, "bearer", refresh_token
                    );

                    return Ok(Redirect::to(redirect_url));
                }

                err => {
                    error!("{:?}", err);

                    return Err(internal_error);
                }
            };
        }

        let mut user = user.unwrap();

        if !user.confirmed && !user_data.verified && !config.auto_confirm {
            user.confirmation_token = Some(secure_token(100));

            user.confirmation_sent_at = Some(Utc::now().naive_utc());

            let user = user.save(&connection);

            if user.is_err() {
                let err = user.err().unwrap();

                error!("{:?}", err);

                return Err(internal_error);
            }

            let user = user.unwrap();

            let confirmation_url = format!(
                "{}/confirm?confirmation_token={}",
                config.instance_url,
                user.confirmation_token.clone().unwrap(),
            );

            let template = email_templates.clone().confirmation_email_template();

            let email = send_confirmation_email(template, confirmation_url, &user, &config);

            if email.is_err() {
                let redirect_url = format!(
                    "{}?error=unable_to_send_confirmation_email",
                    config.site_url
                );

                error!("{:?}", email.err().unwrap());

                return Err(CallbackError::new(redirect_url));
            }

            let redirect_url = format!("{}?error=email_confirmation_required", config.site_url);

            return Ok(Redirect::to(redirect_url));
        }

        let jwt = JWT::new(&user, config.aud.clone());

        let jwt = jwt.sign(config.inner());

        if jwt.is_err() {
            let err = jwt.err().unwrap();

            error!("{:?}", err);

            let redirect_url = format!("{}?error=unable_to_create_access_token", config.site_url);

            return Err(CallbackError::new(redirect_url));
        }

        let jwt = jwt.unwrap();

        let refresh_token = NewRefreshToken::new(user.id);

        let refresh_token = refresh_token.save(&connection);

        if refresh_token.is_err() {
            let err = refresh_token.err();

            error!("{:?}", err);

            let redirect_url = format!("{}?error=unable_to_create_refresh_token", config.site_url);

            return Err(CallbackError::new(redirect_url));
        }

        let refresh_token = refresh_token.unwrap().token;

        let redirect_url = format!(
            "{}?access_token={}&type={}&refresh_token={}",
            config.site_url, jwt, "bearer", refresh_token
        );

        return Ok(Redirect::to(redirect_url));
    });

    if transaction.is_err() {
        let err = transaction.err().unwrap();

        return Redirect::to(err.redirect_url);
    }

    return transaction.unwrap();
}

#[derive(Debug)]
struct CallbackError {
    pub redirect_url: String,
}

impl CallbackError {
    fn new(redirect_url: String) -> CallbackError {
        return CallbackError { redirect_url };
    }
}

impl From<diesel::result::Error> for CallbackError {
    fn from(_: diesel::result::Error) -> Self {
        return CallbackError {
            redirect_url: String::from("unhandled_error"),
        };
    }
}
