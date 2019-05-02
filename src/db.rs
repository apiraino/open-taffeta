use std::env;
use std::ops::Deref;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use crate::models::User;

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

pub fn update_user(conn: &SqliteConnection, user: User) {

    // SupportsReturningClause only available on Postgres
    // On backends which support the RETURNING keyword,
    // foo.save_changes(&conn) is equivalent to
    // update(&foo).set(&foo).get_result(&conn).
    // On other backends, two queries will be executed.

    let update_result : Result<User, diesel::result::Error> = user.save_changes(conn);
    assert_ne!(Err(diesel::NotFound), update_result);
    if let Err(res) = update_result {
        eprintln!("Error in record update: {:?}", res);
    }
}
