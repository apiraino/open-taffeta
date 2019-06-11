use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;

extern crate crypto_hash;
use crypto_hash::{hex_digest, Algorithm};

// Need serde directly, rocket_contrib export is still WIP
use crate::config;
use crate::db;
use crate::schema::userauth;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Debug, Deserialize, Serialize)]
#[table_name = "userauth"]
pub struct Auth {
    // TODO: Sqlite::DateTime in Diesel does not support tz (?)
    pub user_id: i32,
    pub exp: chrono::NaiveDateTime,
    // pub exp: chrono::DateTime<chrono::offset::Utc>,
    pub client_id: String,
    pub token: String,
}

#[derive(Queryable, Debug, Clone)]
pub struct AuthQ {
    pub id: i32,
    pub user_id: i32,
    pub exp: chrono::NaiveDateTime,
    pub client_id: String,
    pub token: String,
}

impl Auth {
    pub fn new(user_id: i32, user_email: &str) -> Auth {
        let exp = get_token_duration!();
        let rndstr = config::generate_password();
        let value = format!("{}{}{}{}{}", exp, user_id, user_email, rndstr, config::get_secret(),);
        let token = hex_digest(Algorithm::SHA1, &value.into_bytes());
        Auth { exp: exp, user_id: user_id, client_id: "client-type-web".to_string(), token: token }
    }
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        let pool = request
            .guard::<State<db::SqlitePool>>()
            .expect("Could not unwrap State in request guard");
        match pool.get() {
            Ok(conn) => {
                if let Some(auth) = extract_auth_from_request(request, db::Conn(conn)) {
                    Outcome::Success(auth)
                } else {
                    Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing))
                }
            }
            Err(_) => {
                eprintln!("Cannot get Db socket conn from pool");
                Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing))
            }
        }
    }
}

pub fn extract_auth_from_request(request: &Request, conn: db::Conn) -> Option<Auth> {
    let header = request.headers().get("authorization").next();
    if let Some(rcvd_token) = header {
        let q = userauth::table.filter(userauth::token.eq(rcvd_token)).get_result(&*conn);
        let auth_record: AuthQ = match q {
            Ok(x) => x,
            Err(err) => {
                eprintln!("get auth failed for token {}; {}", rcvd_token, err);
                return None;
            }
        };

        if !is_expired_token(&auth_record) {
            return None;
        }

        let user_auth: Auth = Auth {
            exp: auth_record.exp,
            user_id: auth_record.user_id,
            client_id: auth_record.client_id,
            token: auth_record.token,
        };
        return Some(user_auth);
    }
    // no token found
    None
}

fn is_expired_token(auth: &AuthQ) -> bool {
    let now = get_now!();
    if now <= auth.exp {
        // eprintln!("Token still valid: {} >= {} ({})", auth.exp, now,
        //           (now <= auth.exp));
        return true;
    }
    // eprintln!("Token has expired: {} >= {} ({})", auth.exp, now,
    //           (now <= auth.exp));
    false
}

pub fn save_auth_token(conn: db::Conn, auth: &Auth) -> Result<(), &str> {
    let insert_res = diesel::insert_into(userauth::table).values(auth).execute(&*conn);
    if let Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) = insert_res {
        eprintln!("auth:save_auth_token Error saving token (Uniqueviolation)");
        return Err("auth:save_auth_token Error saving token (Uniqueviolation)");
    }

    // Count auth tokens
    let auth_tokens: Vec<(i32, String)> = userauth::table
        .select((userauth::id, userauth::exp))
        .filter(userauth::user_id.eq(auth.user_id))
        .order(userauth::exp.asc())
        .load(&*conn)
        .expect(&format!("error getting token count for user id {}", auth.user_id));
    // eprintln!("For user id {} found {} tokens", auth.user_id, auth_tokens.len());
    trim_tokens(conn, auth.user_id, auth_tokens)?;
    Ok(())
}

fn trim_tokens(
    conn: db::Conn,
    user_id: i32,
    auth_tokens: Vec<(i32, String)>,
) -> Result<(), &'static str> {
    // TODO: see if we can refactor into a single delete query

    // Trim expired ones (with tolerance)
    if 1 == !diesel::delete(userauth::table.filter(userauth::exp.lt(get_now!())))
        .execute(&*conn)
        .expect(&format!("error trimming auth tokens for user id {}", user_id))
    {
        return Err("Tokens delete failed (for reasons...)");
    }

    // ... and trim excess
    let curr_token_count = auth_tokens.len() as i64;
    if curr_token_count > config::MAX_AUTH_TOKEN {
        let num_tokens_in_excess = (curr_token_count - config::MAX_AUTH_TOKEN) as usize;
        let tokens_in_excess = &auth_tokens[..num_tokens_in_excess];
        let tokens_ids = tokens_in_excess.iter().map(|(x, _y)| x).collect::<Vec<_>>();
        if 1 == !diesel::delete(userauth::table.filter(userauth::id.eq_any(tokens_ids)))
            .execute(&*conn)
            .expect(&format!("error trimming auth tokens for user id {}", user_id))
        {
            return Err("Tokens delete failed (for reasons...)");
        }
    }
    Ok(())
}
