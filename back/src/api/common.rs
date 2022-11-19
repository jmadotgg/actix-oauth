use actix_web::{post, web::ServiceConfig, HttpResponse};

use super::auth_token::AuthToken;

pub fn api_config(cfg: &mut ServiceConfig) {
    cfg.service(user_info);
}

/// Api request to Github to get user information
#[post("/user_info")]
async fn user_info(auth_token: AuthToken) -> HttpResponse {
    // Just return the decoded auth_token
    HttpResponse::Ok().json(auth_token)
}
