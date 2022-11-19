use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

// ---- Structs
#[derive(Serialize, Deserialize, Debug)]
pub struct OauthResponse {
    pub code: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    scope: String,
    token_type: String,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "AccessTokenRequestError: {}", message)]
pub struct AccessTokenRequestError {
    pub message: String,
}

// ---- Impl blocks
impl actix_web::error::ResponseError for AccessTokenRequestError {}
impl From<reqwest::Error> for AccessTokenRequestError {
    fn from(err: reqwest::Error) -> Self {
        return AccessTokenRequestError {
            message: err.to_string(),
        };
    }
}
