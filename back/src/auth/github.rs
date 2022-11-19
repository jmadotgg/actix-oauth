use actix_files::NamedFile;
use actix_web::{get, http::Uri, post, web, HttpResponse, Responder};

use crate::{
    api::auth_token::TokenResponse, auth::common::encode_token, config::OAuthConfig, User,
};

use super::lib::{AccessTokenRequestError, AccessTokenResponse, OauthResponse};

/// 1. Redirect user to GitHub OAuth Login
#[get("/github")]
async fn github_init(oauth_config: web::Data<OAuthConfig>) -> impl Responder {
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
    secret: web::Data<String>,
) -> Result<web::Json<TokenResponse>, AccessTokenRequestError> {
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

    let access_token_response = res.json::<AccessTokenResponse>().await?;
    let res = client
        .get("https://api.github.com/user")
        // GitHub only permits requests with User-Agent header
        .header("User-Agent", "request")
        .bearer_auth(access_token_response.access_token)
        .send()
        .await?;

    let user = res.json::<User>().await?;
    // Maybe request user information and id to map user to database id
    let id: usize = 34234234234; // generate in database
    let token = encode_token(user.id, &secret);
    Ok(web::Json(TokenResponse { token }))
}
