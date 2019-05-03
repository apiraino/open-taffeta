// Verbatim copy from:
// https://github.com/TatriX/realworld-rust-rocket/blob/master/src/main.rs
use rocket::{Outcome, State};
use rocket::request::{self, FromRequest, Request};
use rocket::http::Status;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;

extern crate crypto_hash;
use crypto_hash::{Algorithm, hex_digest};

// Need serde directly, rocket_contrib export is still WIP
use serde_derive::{Serialize, Deserialize};
use crate::config;
use crate::db::{Conn, SqlitePool};

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,
    pub client_id: String
}

pub type Token = String;

impl Auth {

    pub fn new(id: i32) -> Auth {
        Auth {
            // TODO: return a DateTime
            exp: config::TOKEN_LIFETIME, // get_token_duration!(),
            id: id,
            client_id: "client-type-web".to_string()
        }
    }

    pub fn generate_token(&self, user_email: &str) -> String {
        let rndstr = config::generate_password();
        let value = format!("{}{}{}{}{}",
                            self.exp,
                            self.id,
                            user_email,
                            rndstr,
                            config::get_secret(),
        );
        hex_digest(Algorithm::SHA1, &value.into_bytes())
    }
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {

        let pool = request.guard::<State<SqlitePool>>().expect("FIXME: could not unwrap State");
        match pool.get() {
            Ok(conn) => {
                if let Some(auth) = extract_auth_from_request(request, Conn(conn)) {
                    Outcome::Success(auth)
                } else {
                    Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing))
                }

            }
            Err(_) => {
                eprintln!("Cannot get Db socket conn from pool");
                Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing))
            },
        }
    }
}

fn extract_auth_from_request(request: &Request, conn: Conn) -> Option<Auth> {
    use crate::models::UserAuth;
    use crate::schema::userauth::dsl::*;

    let header = request.headers().get("authorization").next();
    if let Some(rcvd_token) = header {
        eprintln!("DBG (auth::extract_auth_from_request) got token: {}", rcvd_token);

        let user_auth : UserAuth = userauth
            .filter(token.eq(rcvd_token))
            .get_result(&*conn)
            .expect(&format!("get user auth failed for token {}", rcvd_token));

        let a = Auth {
            id: user_auth.user_id,
            exp: config::TOKEN_LIFETIME,
            client_id: "client-type-web".to_string()
        };

        eprintln!("DBG (auth::extract_auth_from_request) user retrieved: {}", a.id);
        return Some(a);
    }
    None
}

pub fn save_auth_token(conn: Conn, fld_user_id: i32, fld_token: &String) {
    use crate::models::UserAuthInsert;
    use crate::schema::userauth::dsl::*;

    // eprintln!("DBG (auth::save_token) self is: {}", self.);
    eprintln!("DBG (auth::save_token) saving token: {}", fld_token);
    // TODO: rotate tokens

    let user_auth = UserAuthInsert {
        user_id: fld_user_id,
        token: fld_token.to_string()
    };

    // TODO: bubble up an exception
    match diesel::insert_into(userauth).values(&user_auth).execute(&*conn) {
        Err(err) => {
            if let diesel::result::Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                _,
            ) = err
            {
                eprintln!("auth:save_auth_token Error saving token (Uniqueviolation)");
            } else {
                eprintln!("auth:save_auth_token Error saving token (other error...)");
            }
        },
        Ok(_) => {
            eprintln!("auth:save_auth_token: Token saved successfully");
        }
    };
}
