use actix_files::NamedFile;
use actix_web::{get, http::Uri, post, web, HttpResponse, Responder};
use reqwest;

use crate::{
    auth::lib::{AccessTokenRequestError, AccessTokenResponse, OauthResponse},
    config::OAuthConfig,
};

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(github)
        .service(github_callback)
        .service(github_access_token);
}

/// 1. Redirect user to GitHub OAuth Login
#[get("/github")]
async fn github(oauth_config: web::Data<OAuthConfig>) -> impl Responder {
    let github_oauth_uri = format!(
        "{}?client_id={}&redirect_uri={}&scope={}&state={}&allow_signup={}",
        oauth_config.oauth_url.clone(),
        oauth_config.client_id,
        oauth_config.callback_url,
        oauth_config.scopes,
        // TODO: Generate state on the fly?
        oauth_config.state,
        oauth_config.allow_signup
    )
    .parse::<Uri>()
    .expect("Invalid URI")
    .to_string();

    println!("{}", github_oauth_uri);

    HttpResponse::Found()
        .insert_header(("Location", github_oauth_uri))
        .finish()
}

/// 2. Callback url from GitHub OAuth dashboard, use code in frontend to request an access token
#[get("/github/callback")]
async fn github_callback() -> Result<NamedFile, actix_web::error::Error> {
    let path = NamedFile::open("front/callback.html")?;
    Ok(path)
}

/// 3. Receive code from frontend and use it to fetch and return an access token
#[post("/github/access_token")]
async fn github_access_token(
    oauth_response: web::Json<OauthResponse>,
    oauth_config: web::Data<OAuthConfig>,
) -> Result<web::Json<AccessTokenResponse>, AccessTokenRequestError> {
    let OauthResponse { code, state } = oauth_response.into_inner();
    if state != oauth_config.state {
        return Err(AccessTokenRequestError {
            message: "Invalid state parameter".to_string(),
        });
    }
    let client = reqwest::Client::new();
    let params = [
        ("client_id", &oauth_config.client_id),
        ("client_secret", &oauth_config.client_secret),
        ("code", &code),
        ("redirect_uri", &oauth_config.callback_url),
    ];
    let res = client
        .post(&oauth_config.access_token_url)
        .form(&params)
        .header("Accept", "application/json")
        .send()
        .await?;

    Ok(web::Json(res.json::<AccessTokenResponse>().await?))
}
