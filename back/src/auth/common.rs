use std::io::Error;

use actix_files::NamedFile;
use actix_web::{get, web};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::api::auth_token::Claims;

use super::github::{github_access_token, github_callback, github_init};

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(github_init)
        .service(github_callback)
        .service(github_access_token);
}

/// Login page with GitHub button
#[get("/login")]
async fn login() -> Result<NamedFile, Error> {
    let path = NamedFile::open("front/login.html")?;
    Ok(path)
}

pub fn encode_token(id: usize, secret: &str) -> String {
    let exp = (Utc::now() + Duration::days(30)).timestamp() as usize;
    let claims = Claims { id, exp };
    // TODO: Error handling
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    token
}
