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
            app_url: env("APP_URL")?.to_string(),
            callback_url: env("CALLBACK_URL")?.to_string(),
            oauth_url: env("OAUTH_URL")?.to_string(),
            access_token_url: env("ACCESS_TOKEN_URL")?.to_string(),
            client_id: env("CLIENT_ID")?.to_string(),
            client_secret: env("CLIENT_SECRET")?.to_string(),
            scopes: env("SCOPES")?.to_string(),
            state: random_state(12),
            allow_signup: env("ALLOW_SIGNUP")?.to_string(),
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
