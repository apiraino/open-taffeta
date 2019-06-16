use log::{error, warn};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};

use crate::auth::token;
use crate::db;
use crate::db::{Conn, SqlitePool};
use crate::models::User;

#[derive(Debug)]
pub enum RoleError {
    Invalid = 0,
    ServerError,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = RoleError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        let pool =
            request.guard::<State<SqlitePool>>().expect("Could not unwrap State in request guard");

        let conn = match pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                error!("Cannot get Db socket conn from pool");
                return Outcome::Failure((Status::Unauthorized, RoleError::ServerError));
            }
        };
        let auth = token::extract_auth_from_request(request, Conn(conn))
            .expect("Could not extract auth token");

        // FIXME: borrower won't let me reuse the DB connection
        let conn = pool.get().unwrap();
        match db::get_user(&conn, auth.user_id) {
            Ok(user) => {
                if user.is_active == false {
                    return Outcome::Failure((Status::Unauthorized, RoleError::ServerError));
                }
                let role = db::get_role(&conn, user.id);
                if user.is_allowed(&request, &role.name, user.id) == false {
                    return Outcome::Failure((Status::Unauthorized, RoleError::ServerError));
                }
                Outcome::Success(user)
            }
            Err(err) => {
                warn!("{}", &format!("Failed to retrieve user profile: {}", err));
                Outcome::Failure((Status::Unauthorized, RoleError::ServerError))
            }
        }
    }
}
