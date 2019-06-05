use std::env;
use std::ops::Deref;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::schema::{roles, users};

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use crate::utils;
use crate::models::{Role, RoleNew, User};
use crate::serializers::user::UserBaseResponse;

// This boilerplate here basically does two things:
// - using r2d2 crate, it creates a pool of DB connections
// - from_request() picks one available connection from the pool and use it to manage SQL queries needed by a request arrived to the server

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL env var"));
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

pub fn get_user(conn: &SqliteConnection, user_id: i32, only_active: Option<bool>)
                        -> Result<User, String>
{
    let user_rs : Result<User, diesel::result::Error> = users::table
        .filter(users::id.eq(user_id))
        .filter(users::is_active.eq(only_active.unwrap_or_else(|| false)))
        .get_result(&*conn);

    match user_rs {
        Ok(records) => {
            return Ok(records);
        },
        Err(err) => {
            let err_msg = format!("user {} retrieval failed: {}",
                                  user_id, err);
            return Err(err_msg);
        }
    };
}

pub fn get_user_list(conn: &SqliteConnection)
                -> Result<Vec<User>, String>
{
    let query_result : Result<Vec<User>, diesel::result::Error> =
        users::table.load(&*conn);

    match query_result {
        Err(err) => {
            let err_msg = format!("error executing query: {}", err);
            return Err(err_msg);
        }
        Ok(vec_user) => { Ok(vec_user) }
    }

    // Ok(query_result)
}
pub fn get_user_profile(conn: &SqliteConnection, user_id: i32)
                -> Result<UserBaseResponse, String>
{
    let query_result : Result<(User, Role), diesel::result::Error> = users::table
        .inner_join(roles::table)
        .filter(users::id.eq(user_id))
        .get_result(&*conn);

    match query_result {
        Err(diesel::NotFound) => {
            let err_msg = format!("error retrieving active user id {}", user_id);
            return Err(err_msg);
        },
        Err(err) => {
            let err_msg = format!("error executing query: {}", err);
            return Err(err_msg);
        }
        Ok((user, role)) =>  {
            // if let Ok(user) = utils::attach_role_to_user(&user, &role) {
            //     // ok
            // }
            let u = utils::attach_role_to_user(&user, &role);
            Ok(u)
        }
    }
}

pub fn update_user(conn: &SqliteConnection, user: User) {

    // SupportsReturningClause only available on Postgres
    // On backends which support the RETURNING keyword,
    // foo.save_changes(&conn) is equivalent to
    // update(&foo).set(&foo).get_result(&conn).
    // On other backends, two queries will be executed.
    // source: http://docs.diesel.rs/diesel/query_dsl/trait.SaveChangesDsl.html#method.save_changes

    let update_result : Result<User, diesel::result::Error> = user.save_changes(conn);
    assert_ne!(Err(diesel::NotFound), update_result);
    if let Err(res) = update_result {
        eprintln!("Error in user update: {:?}", res);
    }
}

pub fn update_role(conn: &SqliteConnection, role: Role) {
    let role_upd_res : Result<Role, diesel::result::Error> =
        role.save_changes(conn);
    if let Err(res) = role_upd_res {
        eprintln!("Error in role update: {:?}", res);
    }
}

pub fn get_role(conn: &SqliteConnection, user_id: i32) -> Role {
    roles::table
        .filter(roles::user.eq(user_id))
        .first(conn)
        .expect("Failed to retrieve role")
}

pub fn add_role(conn: &SqliteConnection, role_data: RoleNew) -> Option<Role> {
    if let Err(err) = diesel::insert_into(roles::table)
         .values(&role_data)
        .execute(conn)
    {
        eprintln!("Role insert failed: {}", err);
        return None;
    }
    Some(get_role(conn, role_data.user.unwrap()))
}

// TODO: centralize in this module all queries
// maybe inside impl User {...}
