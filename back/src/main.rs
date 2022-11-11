use actix_files::NamedFile;
use actix_web::{get, post, web, App, Error, HttpServer};
use back::{auth::auth::auth_config, config::OAuthConfig, OauthApiError, User, UserReq};
use dotenv::dotenv;
use reqwest::Client;
use std::env::var;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_config = (
        var("HOST").expect("HOST not configured"),
        var("PORT")
            .expect("PORT not configured")
            .parse::<u16>()
            .expect("PORT not numeric or out of port range"),
    );
    // Read OAuth details from .env
    let oauth_config =
        web::Data::new(OAuthConfig::new().expect("Missing OAuth config parameter/s"));

    HttpServer::new(move || {
        App::new()
            .app_data(oauth_config.clone())
            .service(
                web::scope("auth")
                    .configure(auth_config)
                    .app_data(oauth_config.clone()),
            )
            .service(index)
            .service(login)
            .service(user_info)
    })
    .bind(server_config)?
    .run()
    .await
}

/// Home page
#[get("/")]
async fn index() -> Result<NamedFile, Error> {
    let path = NamedFile::open("front/index.html")?;
    Ok(path)
}

/// Login page with GitHub button
#[get("/login")]
async fn login() -> Result<NamedFile, Error> {
    let path = NamedFile::open("front/login.html")?;
    Ok(path)
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
