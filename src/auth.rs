// Verbatim copy from:
// https://github.com/TatriX/realworld-rust-rocket/blob/master/src/main.rs
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::http::Status;

extern crate crypto_hash;
use crypto_hash::{Algorithm, hex_digest};

// Need serde directly, rocket_contrib export is still WIP
use serde_derive::{Serialize, Deserialize};
use crate::config;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,
    pub email: String,
    pub client_id: String
}

pub type Token = String;

impl Auth {

    pub fn new(id: i32, email: String) -> Auth {
        Auth {
            // TODO: return a DateTime
            exp: config::TOKEN_LIFETIME, // get_token_duration!(),
            id: id,
            email: email,
            client_id: "client-type-web".to_string()
        }
    }

    pub fn generate_token(&self) -> String {
        let rndstr = config::generate_password();
        let value = format!("{}{}{}{}{}",
                            // TODO: return a DateTime
                            config::TOKEN_LIFETIME, // get_token_duration!(),
                            self.id,
                            self.email.to_string(),
                            rndstr,
                            config::get_secret(),
        );
        hex_digest(Algorithm::SHA1, &value.into_bytes())
    }

    pub fn save_auth_token(&self, token: &String) {
        eprintln!("DBG: saving token: {}", token);
        // TODO: save token
        // TODO: rotate tokens
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
        // TODO: query DB for token
        // then figure out which user is this
        let a = Auth {
            id: 123,
            email: "email@dot.com".to_string(),
            exp: config::TOKEN_LIFETIME,
            client_id: "client-type-web".to_string()
        };
        eprintln!("DBG: got token: {}", token);
        eprintln!("DBG: user retrieved: {}", a.email);
        return Some(a);
    }
    None

    // if let Some(token) = header {
    //     match jwt::decode::<Auth>(
    //         &token.to_string(),
    //         config::get_secret().as_ref(),
    //         &jwt::Validation::new(Algorithm::HS512),
    //     ) {
    //         Err(err) => {
    //             println!("Auth decode error: {:?}", err);
    //         }
    //         Ok(c) => return Some(c.claims),
    //     };
    // }
    // None
}
