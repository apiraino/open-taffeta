use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use rocket::http::Status;

use crate::models::User;
use crate::db;
use crate::auth::token;
use crate::db::{Conn, SqlitePool};

#[derive(Debug)]
pub enum RoleError {
    Invalid = 0,
    ServerError
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = RoleError;
    fn from_request(request: &'a Request<'r>)
                    -> request::Outcome<User, Self::Error>
    {
        let pool = request
            .guard::<State<SqlitePool>>()
            .expect("Could not unwrap State in request guard");

        let conn = match pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                eprintln!("Cannot get Db socket conn from pool");
                return Outcome::Failure(
                    (Status::Unauthorized, RoleError::ServerError)
                );
            }
        };
        let auth = token::extract_auth_from_request(request, Conn(conn))
            .expect("Could not extract auth token");

        // FIXME: borrower won't let me reuse the DB connection
        let conn = pool.get().unwrap();
        match db::get_user(&conn, auth.user_id, Some(true)) {
            Ok(user) => {
                let role = db::get_role(&conn, user.id);
                if user.is_allowed(&request, &role.name) == false
                {
                    return Outcome::Failure (
                        (Status::Unauthorized, RoleError::ServerError)
                    );
                }
                Outcome::Success(user)
            },
            Err(err) => {
                eprintln!("{}",
                    &format!("Failed to retrieve user profile: {}", err)
                );
                Outcome::Failure((Status::Unauthorized, RoleError::ServerError))
            }
        }
    }
}
