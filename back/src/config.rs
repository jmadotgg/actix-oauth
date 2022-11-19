use rand::{distributions::Alphanumeric, Rng};

use std::env::{var as env, VarError};

#[derive(Debug)]
pub struct OAuthConfig {
    pub app_url: String,
    pub callback_url: String,
    pub oauth_url: String,
    pub access_token_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub scopes: String,
    pub state: String,
    pub allow_signup: String,
}

impl OAuthConfig {
    pub fn new() -> Result<OAuthConfig, VarError> {
        let oauth_config = OAuthConfig {
            app_url: env("APP_URL")?,
            callback_url: env("CALLBACK_URL")?,
            oauth_url: env("OAUTH_URL")?,
            access_token_url: env("ACCESS_TOKEN_URL")?,
            client_id: env("CLIENT_ID")?,
            client_secret: env("CLIENT_SECRET")?,
            scopes: env("SCOPES")?,
            state: random_state(12),
            allow_signup: env("ALLOW_SIGNUP")?,
        };
        Ok(oauth_config)
    }
}

fn random_state(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
