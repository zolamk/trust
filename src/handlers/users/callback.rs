#[get("/authorize/callback?<code>&<state>")]
pub fn callback(config: State<Config>, code: String, state: String) -> ProviderResponse {
    let state = ProviderState::verify(state, config.inner());

    if state.is_err() {
        let err = state.err().unwrap();

        error!("{:?}", err);

        let redirect_url = format!("{}?error=invalid_state", config.site_url);

        return ProviderResponse::Redirect(Redirect::to(redirect_url));
    }

    let client = facebook_client(config.clone());

    if client.is_err() {
        let redirect_url = format!("{}?error=internal_error", config.site_url);

        return ProviderResponse::Redirect(Redirect::to(redirect_url));
    }

    let client = client.unwrap();

    let access_token = client
        .exchange_code(AuthorizationCode::new(code))
        .request(http_client);

    if access_token.is_err() {
        let err = access_token.err().unwrap();

        error!("{:?}", err);

        let redirect_url = format!("{}?error=error_exchanging_code", config.site_url);

        return ProviderResponse::Redirect(Redirect::to(redirect_url));
    }

    let access_token = access_token.unwrap();

    let client = reqwest::Client::new();

    let profile_url = Url::parse("https://graph.facebook.com/me?fields=email,name").unwrap();

    let response = client
        .get(profile_url.into_string().as_str())
        .bearer_auth(access_token.access_token().secret())
        .send();

    if response.is_err() {
        let err = response.err().unwrap();

        error!("{:?}", err);

        let redirect_url = format!("{}?error=error_fetching_profile_info", config.site_url);

        return ProviderResponse::Redirect(Redirect::to(redirect_url));
    }

    let response = response.unwrap();

    println!("{:?}", response);

    return ProviderResponse::Other(Err(Error {
        code: 403,
        body: json!({ "body": "test" }),
    }));
}
