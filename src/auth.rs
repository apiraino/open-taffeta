// Verbatim copy from:
// https://github.com/TatriX/realworld-rust-rocket/blob/master/src/main.rs
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::http::Status;
// Need serde directly, rocket_contrib export is still WIP
use serde_derive::{Serialize, Deserialize};
use jsonwebtoken as jwt;
use self::jwt::{Header, Algorithm};
use crate::config;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,
    pub email: String,
}

impl Auth {
    pub fn token(&self) -> String {
        let mut header = Header::default();
        header.kid = Some("TODO: signing_key".to_owned());
        header.alg = Algorithm::HS512;
        let payload = Auth {
            exp: 10_000_000_000,
            id: self.id,
            email: self.email.to_string()
        };
        jwt::encode(
            &header,
            &payload,
            config::get_secret().as_ref()
        ).expect("jwt encoding failed")
    }
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        if let Some(auth) = extract_auth_from_request(request) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing))
        }
    }
}

fn extract_auth_from_request(request: &Request) -> Option<Auth> {
    let header = request.headers().get("authorization").next();
    if let Some(token) = header {
        match jwt::decode::<Auth>(
            &token.to_string(),
            config::get_secret().as_ref(),
            &jwt::Validation::new(Algorithm::HS512),
        ) {
            Err(err) => {
                println!("Auth decode error: {:?}", err);
            }
            Ok(c) => return Some(c.claims),
        };
    }
    None
}
