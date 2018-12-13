// Verbatim copy from:
// https://github.com/TatriX/realworld-rust-rocket/blob/master/src/main.rs
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::http::Status;
// Need serde directly, rocket_contrib export is still WIP
use serde_derive::{Serialize, Deserialize};
use crate::config;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,
    pub username: String,
}

impl Auth {
    pub fn token(&self) -> String {
        let header = json!({});
        let payload = json!(self);
        frank_jwt::encode(
            header,
            &config::SECRET.to_string(),
            &payload,
            frank_jwt::Algorithm::HS256,
        ).expect("frank_jwt")
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
        match frank_jwt::decode(
            &token.to_string(),
            &config::SECRET.to_string(),
            frank_jwt::Algorithm::HS256,
        ) {
            Err(err) => {
                println!("Auth decode error: {:?}", err);
            }
            Ok((_, payload)) => match serde_json::from_value::<Auth>(payload) {
                Ok(auth) => return Some(auth),
                Err(err) => println!("Auth serde decode error: {:?}", err),
            },
        };
    }
    None
}
