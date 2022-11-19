use std::future::{ready, Ready};

use actix_web::{
    dev::Payload,
    error::ErrorUnauthorized,
    http::{self, header::HeaderValue},
    web, Error as ActixWebError, FromRequest, HttpRequest,
};
use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthToken {
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

impl FromRequest for AuthToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // get auth token from authorization header
        let auth_header: Option<&HeaderValue> = req.headers().get(http::header::AUTHORIZATION);

        if auth_header.is_none() {
            return ready(Err(ErrorUnauthorized("No auth header provided")));
        }

        let auth_token = auth_header.unwrap().to_str().unwrap_or("");
        println!("{}", auth_token);

        if auth_token.is_empty() {
            return ready(Err(ErrorUnauthorized("Invalid auth token!")));
        }

        // TODO: handle correctly
        let secret = req.app_data::<web::Data<String>>().expect("No secret");

        let decode: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
            auth_token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => ready(Ok(AuthToken {
                id: token.claims.id,
            })),
            Err(_) => ready(Err(ErrorUnauthorized("Unauthorized"))),
        }
    }
}
