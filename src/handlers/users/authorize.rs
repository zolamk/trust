use crate::{
    config::Config,
    handlers::{
        users::provider::{FacebookProvider, GithubProvider, GoogleProvider, Provider, ProviderResponse, ProviderState},
        Error,
    },
    operator_signature::{Error as OperatorSignatureError, OperatorSignature},
};
use log::error;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use rocket::{response::Redirect, State};

#[get("/authorize?<provider>")]
pub fn authorize(config: State<Config>, provider: String, operator_signature: Result<OperatorSignature, OperatorSignatureError>) -> ProviderResponse {
    let internal_error = Err(Error::new(
        500,
        json!({
            "code": "internal_error",
        }),
        "Internal Server Error".to_string(),
    ));

    if operator_signature.is_err() {
        let err = operator_signature.err().unwrap();

        error!("{:?}", err);

        return ProviderResponse::Other(internal_error);
    }

    let provider_disabled = ProviderResponse::Other(Err(Error::new(
        400,
        json!({
            "code": "provider_disabled",
        }),
        "Provider Disable".to_string(),
    )));

    let oauth_provider: Box<dyn Provider> = match provider.as_str() {
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
            return ProviderResponse::Other(Err(Error::new(
                400,
                json!({
                    "code": "invalid_provider",
                }),
                "Invalid Provider".to_string(),
            )))
        }
    };

    let client_id = oauth_provider.client_id();

    let client_id = ClientId::new(client_id);

    let client_secret = oauth_provider.client_secret();

    let client_secret = Some(ClientSecret::new(client_secret));

    let auth_url = oauth_provider.auth_url();

    let auth_url = AuthUrl::new(auth_url);

    if auth_url.is_err() {
        let err = auth_url.err().unwrap();

        error!("{:?}", err);

        return ProviderResponse::Other(internal_error);
    }

    let token_url = oauth_provider.token_url();

    let token_url = TokenUrl::new(token_url);

    if token_url.is_err() {
        let err = token_url.err().unwrap();

        error!("{:?}", err);

        return ProviderResponse::Other(internal_error);
    }

    let token_url = Some(token_url.unwrap());

    let redirect_url = format!("{}/authorize/callback", config.instance_url);

    let redirect_url = RedirectUrl::new(redirect_url);

    if redirect_url.is_err() {
        let err = redirect_url.err().unwrap();

        error!("{:?}", err);

        return ProviderResponse::Other(internal_error);
    }

    let client = BasicClient::new(client_id, client_secret, auth_url.unwrap(), token_url).set_redirect_url(redirect_url.unwrap());

    let state = ProviderState::new(provider);

    let state = state.sign(config.inner());

    if state.is_err() {
        let err = state.err().unwrap();

        error!("{:?}", err);

        return ProviderResponse::Other(internal_error);
    }

    let state = state.unwrap();

    let mut auth_url = client.authorize_url(|| CsrfToken::new(state));

    for scope in oauth_provider.scopes().iter() {
        auth_url = auth_url.add_scope(Scope::new(scope.clone()));
    }

    let (auth_url, _) = auth_url.url();

    return ProviderResponse::Redirect(Redirect::to(auth_url.into_string()));
}
