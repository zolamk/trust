use crate::{
    config::Config,
    crypto::{jwt::JWT, secure_token},
    handlers::users::provider::{FacebookProvider, GithubProvider, GoogleProvider, Provider, ProviderState},
    hook::{HookEvent, Webhook},
    mailer::{send_email, EmailTemplates},
    models::{
        refresh_token::NewRefreshToken,
        user::{get_by_email, NewUser, User},
        Error as ModelError,
    },
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use chrono::Utc;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    result::{
        DatabaseErrorKind,
        Error::{DatabaseError, NotFound},
    },
    Connection,
};
use log::error;
use oauth2::{basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl};
use rocket::{response::Redirect, State};

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

    let provider_disabled = Redirect::to(format!("{}?error=provider_disabled", operator_signature.site_url));

    let internal_error = Redirect::to(format!("{}?error=internal_error", operator_signature.site_url));

    let connection = match connection_pool.get() {
        Ok(connection) => connection,
        Err(_err) => {
            return internal_error;
        }
    };

    let oauth_provider: Box<dyn Provider> = match state.provider.as_str() {
        "facebook" => {
            if !config.facebook_enabled {
                return provider_disabled;
            }

            Box::new(FacebookProvider::new(config.inner().clone()))
        }
        "google" => {
            if !config.google_enabled {
                return provider_disabled;
            }

            Box::new(GoogleProvider::new(config.inner().clone()))
        }
        "github" => {
            if !config.github_enabled {
                return provider_disabled;
            }
            Box::new(GithubProvider::new(config.inner().clone()))
        }
        _ => {
            let redirect_url = format!("{}?error=invalid_provider", operator_signature.site_url);

            return Redirect::to(redirect_url);
        }
    };

    let client_id = oauth_provider.client_id();

    let client_id = ClientId::new(client_id);

    let client_secret = oauth_provider.client_secret();

    let client_secret = Some(ClientSecret::new(client_secret));

    let auth_url = AuthUrl::new(oauth_provider.auth_url());

    if auth_url.is_err() {
        let err = auth_url.err().unwrap();

        error!("{:?}", err);

        return internal_error;
    }

    let token_url = TokenUrl::new(oauth_provider.token_url());

    if token_url.is_err() {
        let err = token_url.err().unwrap();

        error!("{:?}", err);

        return internal_error;
    }

    let token_url = Some(token_url.unwrap());

    let redirect_url = format!("{}/authorize/callback", config.instance_url);

    let redirect_url = RedirectUrl::new(redirect_url);

    if redirect_url.is_err() {
        let err = redirect_url.err().unwrap();

        error!("{:?}", err);

        return internal_error;
    }

    let client = BasicClient::new(client_id, client_secret, auth_url.unwrap(), token_url).set_redirect_url(redirect_url.unwrap());

    let access_token = client.exchange_code(AuthorizationCode::new(code)).request(http_client);

    if access_token.is_err() {
        let err = access_token.err().unwrap();

        error!("{:?}", err);

        let redirect_url = format!("{}?error=error_exchanging_code", operator_signature.site_url);

        return Redirect::to(redirect_url);
    }

    let access_token = access_token.unwrap();

    let access_token = access_token.access_token().secret();

    let user_data = oauth_provider.get_user_data(access_token.clone());

    if user_data.is_err() {
        let err = user_data.err().unwrap();

        error!("{:?}", err);

        let redirect_url = format!("{}?error=error_getting_user_data", operator_signature.site_url);

        return Redirect::to(redirect_url);
    }

    let user_data = user_data.unwrap();

    if user_data.email.is_none() {
        let redirect_url = format!("{}?error=unable_to_find_email_with_provider", operator_signature.site_url);

        return Redirect::to(redirect_url);
    }

    let email = user_data.email.clone().unwrap();

    let transaction = connection.transaction::<Redirect, CallbackError, _>(|| {
        let u = get_by_email(email.clone(), &connection);

        let mut user: User;

        let hook_response: Option<serde_json::Value>;

        let internal_error_redirect_url = format!("{}?error=internal_error", operator_signature.site_url);

        // there was an error finding the user
        if u.is_err() {
            let err = u.err().unwrap();

            // if the error was the user doesn't exist
            if let ModelError::DatabaseError(NotFound) = err {
                if config.disable_signup {
                    let redirect_url = format!("{}?error=signup_disabled", operator_signature.site_url);

                    return Err(CallbackError::new(redirect_url));
                }

                let new_user = NewUser {
                    email,
                    confirmed: user_data.verified || config.auto_confirm,
                    name: user_data.name,
                    avatar: user_data.avatar,
                    confirmation_token_sent_at: None,
                    confirmation_token: None,
                    invitation_sent_at: None,
                    is_admin: false,
                    password: None,
                };

                let u = new_user.save(&connection);

                if u.is_err() {
                    let err = u.err().unwrap();

                    if let ModelError::DatabaseError(DatabaseError(DatabaseErrorKind::UniqueViolation, _info)) = err {
                        let redirect_url = format!("{}?error=email_already_registered", operator_signature.site_url);

                        return Err(CallbackError::new(redirect_url));
                    }

                    error!("{:?}", err);

                    return Err(CallbackError::new(internal_error_redirect_url));
                }

                user = u.unwrap();

                let hook_payload = json!({
                    "event": HookEvent::Signup,
                    "provider": state.provider,
                    "user": user,
                });

                let hook = Webhook::new(HookEvent::Signup, hook_payload, config.clone(), operator_signature.clone());

                let hr = hook.trigger();

                if hr.is_err() {
                    let redirect_url = format!("{}?error=signup_hook_error", operator_signature.site_url);

                    return Err(CallbackError::new(redirect_url));
                }

                hook_response = hr.unwrap();
            }
            error!("{:?}", err);

            return Err(CallbackError::new(internal_error_redirect_url));
        } else {
            user = u.unwrap();

            let hook_payload = json!({
                "event": HookEvent::Login,
                "provider": state.provider,
                "user": user,
            });

            let hook = Webhook::new(HookEvent::Login, hook_payload, config.clone(), operator_signature.clone());

            let hr = hook.trigger();

            if hr.is_err() {
                let redirect_url = format!("{}?error=login_hook_error", operator_signature.site_url);

                return Err(CallbackError::new(redirect_url));
            }

            hook_response = hr.unwrap();
        }

        if !user.confirmed {
            if !user_data.verified && !config.auto_confirm {
                user.confirmation_token = Some(secure_token(100));

                user.confirmation_token_sent_at = Some(Utc::now().naive_utc());

                let user = user.save(&connection);

                if user.is_err() {
                    let err = user.err().unwrap();

                    error!("{:?}", err);

                    return Err(CallbackError::new(internal_error_redirect_url));
                }

                let user = user.unwrap();

                let template = email_templates.clone().confirmation_email_template();

                let data = json!({
                    "confirmation_url": format!("{}/confirm?confirmation_token={}", config.instance_url, user.confirmation_token.clone().unwrap()),
                    "site_url": config.site_url,
                    "email": user.email
                });

                let email = send_email(template, data, &user, &config);

                if email.is_err() {
                    let redirect_url = format!("{}?error=unable_to_send_confirmation_email", operator_signature.site_url);

                    error!("{:#?}", email.err().unwrap());

                    return Err(CallbackError::new(redirect_url));
                }

                let redirect_url = format!("{}?error=email_confirmation_required", operator_signature.site_url);

                return Ok(Redirect::to(redirect_url));
            }

            if user.confirm(&connection).is_err() {
                return Ok(Redirect::to(internal_error_redirect_url));
            }
        }

        let jwt = JWT::new(&user, config.aud.clone(), hook_response);

        let jwt = jwt.sign(config.inner());

        if jwt.is_err() {
            let err = jwt.err().unwrap();

            error!("{:#?}", err);

            let redirect_url = format!("{}?error=unable_to_create_access_token", operator_signature.site_url);

            return Err(CallbackError::new(redirect_url));
        }

        let jwt = jwt.unwrap();

        let refresh_token = NewRefreshToken::new(user.id);

        let refresh_token = refresh_token.save(&connection);

        if refresh_token.is_err() {
            let err = refresh_token.err();

            error!("{:#?}", err);

            let redirect_url = format!("{}?error=unable_to_create_refresh_token", operator_signature.site_url);

            return Err(CallbackError::new(redirect_url));
        }

        let refresh_token = refresh_token.unwrap().token;

        let redirect_url = format!("{}?access_token={}&type={}&refresh_token={}", operator_signature.site_url, jwt, "bearer", refresh_token);

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
