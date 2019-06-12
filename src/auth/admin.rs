use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};

use crate::auth::token;
use crate::auth::user::RoleError;
use crate::db;
use crate::db::{Conn, SqlitePool};

pub struct AdminUser {
    pub id: i32,
    pub email: String,
    pub is_active: bool,
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = RoleError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser, Self::Error> {
        let pool =
            request.guard::<State<SqlitePool>>().expect("Could not unwrap State in request guard");

        let conn = match pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                eprintln!("Cannot get Db socket conn from pool");
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

                let admin_user =
                    AdminUser { id: user.id, email: user.email, is_active: user.is_active };
                Outcome::Success(admin_user)
            }
            Err(err) => {
                eprintln!("{}", &format!("Failed to retrieve user profile: {}", err));
                Outcome::Failure((Status::Unauthorized, RoleError::ServerError))
            }
        }
    }
}
