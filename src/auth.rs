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
use crate::schema::userauth;

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize)]
#[table_name = "userauth"]
pub struct Auth {
    // TODO: Sqlite::DateTime in Diesel does not support tz (?)
    // pub exp: chrono::DateTime<chrono::offset::Utc>,
    pub user_id: i32,
    pub exp: chrono::NaiveDateTime,
    pub client_id: String,
    pub token: String
}

#[derive(Queryable)]
pub struct AuthQ {
    pub id: i32,
    pub user_id: i32,
    pub exp: chrono::NaiveDateTime,
    pub client_id: String,
    pub token: String
}

pub type Token = String;

impl Auth {

    pub fn new(user_id: i32, user_email: &str) -> Auth {
        let exp = get_token_duration!();
        let rndstr = config::generate_password();
        let value = format!("{}{}{}{}{}",
                            exp,
                            user_id,
                            user_email,
                            rndstr,
                            config::get_secret(),
        );
        let token = hex_digest(Algorithm::SHA1, &value.into_bytes());
        Auth {
            exp: exp,
            user_id: user_id,
            client_id: "client-type-web".to_string(),
            token: token
        }
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
    use crate::schema::userauth::dsl::*;

    let header = request.headers().get("authorization").next();
    if let Some(rcvd_token) = header {
        eprintln!("DBG (auth::extract_auth_from_request) got token: {}", rcvd_token);

        let user_auth : AuthQ = userauth
            .filter(token.eq(rcvd_token))
            .get_result(&*conn)
            .expect(&format!("get user auth failed for token {}", rcvd_token));

        // TODO: check against != 1 records returned

        eprintln!("DBG (auth::extract_auth_from_request) user retrieved: {}", user_auth.user_id);

        let user_auth_q : Auth = Auth {
            exp: user_auth.exp,
            user_id: user_auth.user_id,
            client_id: user_auth.client_id,
            token: user_auth.token
        };
        return Some(user_auth_q);
    }
    None
}

pub fn save_auth_token(conn: Conn, auth: &Auth) {
    use crate::schema::userauth::dsl::*;

    eprintln!("DBG (auth::save_token) saving token: {}", auth.token);
    // TODO: rotate tokens

    // TODO: bubble up an exception
    match diesel::insert_into(userauth).values(auth).execute(&*conn) {
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
