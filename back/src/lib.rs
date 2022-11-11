use derive_more::Display;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

pub mod auth;
pub mod config;

// ---- Structs
#[derive(Debug, Display, derive_more::Error)]
#[display(fmt = "ApiError: {}", message)]
pub struct OauthApiError {
    pub message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserReq {
    pub access_token: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u32,
    name: String,
    location: String,
    email: Option<String>,
    login: String,
    avatar_url: String,
}

// ---- Impl blocks
impl actix_web::error::ResponseError for OauthApiError {
    fn status_code(&self) -> reqwest::StatusCode {
        // Means that GITHUB api requests are rate limited
        StatusCode::UNAUTHORIZED
    }
}
impl From<reqwest::Error> for OauthApiError {
    fn from(err: reqwest::Error) -> Self {
        return OauthApiError {
            message: err.to_string(),
        };
    }
}
