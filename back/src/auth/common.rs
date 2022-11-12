use std::io::Error;

use actix_files::NamedFile;
use actix_web::{get, web};

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
