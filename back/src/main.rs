use actix_files::NamedFile;
use actix_web::{get, web, App, Error, HttpServer};
use back::{api::common::api_config, auth::common::auth_config, config::OAuthConfig};
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
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

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(
                web::scope("auth")
                    .configure(auth_config)
                    .app_data(oauth_config.clone()),
            )
            .service(web::scope("api").configure(api_config))
    })
    .bind_openssl(server_config, builder)?
    .run()
    .await
}

/// Home page
#[get("/")]
async fn index() -> Result<NamedFile, Error> {
    let path = NamedFile::open("front/index.html")?;
    Ok(path)
}
