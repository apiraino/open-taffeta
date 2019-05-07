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
    pub user_id: i32,
    pub exp: chrono::NaiveDateTime,
    // pub exp: chrono::DateTime<chrono::offset::Utc>,
    pub client_id: String,
    pub token: String
}

#[derive(Queryable, Debug, Clone)]
pub struct AuthQ {
    pub id: i32,
    pub user_id: i32,
    pub exp: chrono::NaiveDateTime,
    pub client_id: String,
    pub token: String
}

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
    let header = request.headers().get("authorization").next();
    if let Some(rcvd_token) = header {
        let auth_record : AuthQ = userauth::table
            .filter(userauth::token.eq(rcvd_token))
            .get_result(&*conn)
            .expect(&format!("get auth failed for token {}", rcvd_token));

        if is_valid_token(&auth_record) {
            let user_auth : Auth = Auth {
                exp: auth_record.exp,
                user_id: auth_record.user_id,
                client_id: auth_record.client_id,
                token: auth_record.token
            };
            return Some(user_auth);
        }
    }
    None
}

fn is_valid_token(auth: &AuthQ) -> bool {
    let now = get_now!();
    if now <= auth.exp {
        eprintln!(">>> token still valid: {} >= {} ({})", auth.exp, now,
                  (now <= auth.exp));
        return true;
    } else {
        eprintln!(">>> Token has expired: {} >= {} ({})", auth.exp, now,
                  (now <= auth.exp));
    }
    false
}

pub fn save_auth_token(conn: Conn, auth: &Auth) -> Result<(), &str> {

    // TODO: trim expired tokens

    // match diesel::insert_into(userauth).values(auth).execute(&*conn) {
    //     Err(err) => {
    //         if let diesel::result::Error::DatabaseError(
    //             DatabaseErrorKind::UniqueViolation,
    //             _,
    //         ) = err
    //         {
    //             eprintln!("auth:save_auth_token Error saving token (Uniqueviolation)");
    //         } else {
    //             eprintln!("auth:save_auth_token Error saving token (other error...)");
    //         }
    //     },
    //     Ok(_) => {
    //         eprintln!("auth:save_auth_token: Token saved successfully");
    //     }
    // };

    let insert_res = diesel::insert_into(userauth::table).values(auth).execute(&*conn);
    if let Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation,_,)) = insert_res {
        eprintln!("auth:save_auth_token Error saving token (Uniqueviolation)");
        return Err("auth:save_auth_token Error saving token (Uniqueviolation)");
    }

    // Count auth tokens
    let auth_tokens : Vec<i32> = userauth::table
        .select(userauth::id)
        .filter(userauth::user_id.eq(auth.user_id))
        .order(userauth::exp.asc())
        .load(&*conn)
        .expect(&format!(
            "error getting token count for user id {}",
            auth.user_id
        ));
    eprintln!(">>> For user id {} found {} tokens", auth.user_id, auth_tokens.len());

    // ... and trim expired ones
    // let expiry = chrono::NaiveDateTime
    // let delete_res = diesel::delete(
    //     userauth::table.filter(
    //         userauth::exp.ge_any(tokens_in_excess)
    //     )
    // )
    //     .execute(&*conn)
    //     .expect(&format!(
    //         "error trimming auth tokens for user id {}",
    //         auth.user_id
    //     ));

    // ... and trim excess
    let curr_token_count = auth_tokens.len() as i64;
    if curr_token_count > config::MAX_AUTH_TOKEN {
        let num_tokens_in_excess = (curr_token_count - config::MAX_AUTH_TOKEN) as usize;
        let tokens_in_excess = &auth_tokens[..num_tokens_in_excess];

        let delete_res = diesel::delete(
            userauth::table.filter(
                userauth::id.eq_any(tokens_in_excess)
            )
        )
            .execute(&*conn)
            .expect(&format!(
                "error trimming auth tokens for user id {}",
                auth.user_id
            ));
        eprintln!("Delete result: {}", delete_res);
    }
    Ok(())
}
