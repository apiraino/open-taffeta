use log::{debug, error};
use std::env;
use std::ops::Deref;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use crate::models::{Role, RoleNew, User};
use crate::schema::{roles, users};
use crate::serializers::user::UserBaseResponse;
use crate::utils;

// This boilerplate here basically does two things:
// - using r2d2 crate, it creates a pool of DB connections
// - from_request() picks one available connection from the pool and use it to manage SQL queries needed by a request arrived to the server

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> SqlitePool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var");
    debug!("DB initialized from url: {}", db_url);
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    Pool::new(manager).expect("db pool")
}

pub struct Conn(pub PooledConnection<ConnectionManager<SqliteConnection>>);

impl Deref for Conn {
    type Target = SqliteConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

pub fn get_user(conn: &SqliteConnection, user_id: i32) -> Result<User, String> {
    let user_rs: Result<User, diesel::result::Error>;
    user_rs = users::table.filter(users::id.eq(user_id)).get_result(&*conn);

    match user_rs {
        Ok(records) => {
            return Ok(records);
        }
        Err(err) => {
            let err_msg = format!("user {} retrieval failed: {}", user_id, err);
            return Err(err_msg);
        }
    };
}

pub fn get_user_list(
    conn: &SqliteConnection,
    only_active: bool,
) -> Result<Vec<(User, Role)>, String> {
    let query_result: Result<Vec<(User, Role)>, diesel::result::Error>;
    if only_active {
        query_result =
            users::table.inner_join(roles::table).filter(users::is_active.eq(true)).load(&*conn);
    } else {
        query_result = users::table.inner_join(roles::table).load(&*conn);
    }

    match query_result {
        Err(err) => {
            let err_msg = format!("error executing query: {}", err);
            return Err(err_msg);
        }
        Ok(vec_user) => Ok(vec_user),
    }
}

pub fn get_active_user(
    conn: &SqliteConnection,
    pass: &str,
    email: &str,
) -> Result<(User, Role), String> {
    let query_result: Result<(User, Role), diesel::result::Error> = users::table
        .inner_join(roles::table)
        .filter(users::is_active.eq(true))
        .filter(users::email.eq(email))
        .filter(users::password.eq(pass))
        .get_result(&*conn);

    match query_result {
        Err(diesel::NotFound) => {
            let err_msg = format!("error retrieving active user email {}", email);
            return Err(err_msg);
        }
        Err(err) => {
            let err_msg = format!("error executing query: {}", err);
            return Err(err_msg);
        }
        Ok((user, role)) => Ok((user, role)),
    }
}

pub fn get_user_profile(conn: &SqliteConnection, email: &str) -> Result<UserBaseResponse, String> {
    let query_result: Result<(User, Role), diesel::result::Error> =
        users::table.inner_join(roles::table).filter(users::email.eq(email)).get_result(&*conn);

    match query_result {
        Err(diesel::NotFound) => {
            let err_msg = format!("error retrieving active user email {}", email);
            return Err(err_msg);
        }
        Err(err) => {
            let err_msg = format!("error executing query: {}", err);
            return Err(err_msg);
        }
        Ok((user, role)) => {
            let u = utils::attach_role_to_user(&user, &role);
            Ok(u)
        }
    }
}

pub fn update_user(conn: &SqliteConnection, user: &User) -> Result<User, String> {
    // SupportsReturningClause only available on Postgres
    // On backends which support the RETURNING keyword,
    // foo.save_changes(&conn) is equivalent to
    // update(&foo).set(&foo).get_result(&conn).
    // On other backends, two queries will be executed.
    // source: http://docs.diesel.rs/diesel/query_dsl/trait.SaveChangesDsl.html#method.save_changes

    let update_result: Result<User, diesel::result::Error> = user.save_changes(conn);
    assert_ne!(Err(diesel::NotFound), update_result);
    if let Err(res) = update_result {
        return Err(format!("Error in user update: {:?}", res));
    }
    Ok(update_result.unwrap())
}

pub fn update_role(conn: &SqliteConnection, role: Role) -> Result<Role, String> {
    // TODO: ensure no one sets roles to 🍆
    let role_upd_res: Result<Role, diesel::result::Error> = role.save_changes(conn);
    if let Err(res) = role_upd_res {
        return Err(format!("Error in role update: {:?}", res));
    }
    Ok(role_upd_res.unwrap())
}

pub fn get_role(conn: &SqliteConnection, user_id: i32) -> Role {
    roles::table.filter(roles::user.eq(user_id)).first(conn).expect("Failed to retrieve role")
}

pub fn add_role(conn: &SqliteConnection, role_data: RoleNew) -> Option<Role> {
    if let Err(err) = diesel::insert_into(roles::table).values(&role_data).execute(conn) {
        error!("Role insert failed: {}", err);
        return None;
    }
    Some(get_role(conn, role_data.user.unwrap()))
}

// TODO: centralize in this module all queries
// maybe inside impl User {...}
