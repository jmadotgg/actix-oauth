use actix_web::{
    post,
    web::{self, ServiceConfig},
};
use reqwest::Client;

use crate::{OauthApiError, User, UserReq};

pub fn api_config(cfg: &mut ServiceConfig) {
    cfg.service(user_info);
}

/// Api request to Github to get user information
#[post("/user_info")]
async fn user_info(user_req: web::Json<UserReq>) -> Result<web::Json<User>, OauthApiError> {
    let client = Client::new();
    if user_req.access_token.is_none() {
        return Err(OauthApiError {
            message: "No access token provided".to_string(),
        });
    }
    let res = client
        .get("https://api.github.com/user")
        // GitHub only permits requests with User-Agent header
        .header("User-Agent", "request")
        .bearer_auth(user_req.access_token.clone().unwrap())
        .send()
        .await?;

    Ok(web::Json(res.json::<User>().await?))
}
