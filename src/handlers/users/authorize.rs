extern crate oauth2;
extern crate reqwest;
extern crate rocket;

use crate::config::Config;
use crate::error::Error;
use crate::handlers::users::provider::FacebookProvider;
use crate::handlers::users::provider::Provider;
use crate::handlers::users::provider::ProviderResponse;
use crate::handlers::users::provider::ProviderState;
use log::error;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use rocket::response::Redirect;
use rocket::State;
use url::Url;

#[get("/authorize?<provider>")]
pub fn authorize(config: State<Config>, provider: String) -> ProviderResponse {
    let internal_error = Err(Error {
        code: 500,
        body: json!({
            "code": "internal_error",
        }),
    });

    let provider_disabled = Err(Error {
        code: 400,
        body: json!({
            "code": "provider_disabled",
        }),
    });

    let oauth_provider = match provider.as_str() {
        "facebook" => {
            if !config.facebook_enabled {
                return ProviderResponse::Other(provider_disabled);
            }
            Box::new(FacebookProvider::new(config.inner().clone()))
        }
        _ => {
            return ProviderResponse::Other(Err(Error {
                code: 400,
                body: json!({
                    "code": "invalid_provider",
                }),
            }))
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

        return ProviderResponse::Other(internal_error);
    }

    let auth_url = AuthUrl::new(auth_url.unwrap());

    let token_url = oauth_provider.clone().token_url();

    let token_url = Url::parse(token_url.as_str());

    if token_url.is_err() {
        let err = token_url.err().unwrap();

        error!("{:?}", err);

        return ProviderResponse::Other(internal_error);
    }

    let token_url = Some(TokenUrl::new(token_url.unwrap()));

    let redirect_url = format!("{}/authorize/callback", config.instance_url);

    let redirect_url = Url::parse(redirect_url.as_str());

    if redirect_url.is_err() {
        let err = redirect_url.err().unwrap();

        error!("{:?}", err);

        return ProviderResponse::Other(internal_error);
    }

    let redirect_url = RedirectUrl::new(redirect_url.unwrap());

    let client = BasicClient::new(client_id, client_secret, auth_url, token_url)
        .set_redirect_url(redirect_url);

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
