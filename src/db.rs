use std::ops::Deref;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

// This boilerplate here basically does two things:
// - using r2d2 crate, it creates a pool of DB connections
// - from_request() picks one available connection from the pool and use it to manage SQL queries needed by a request arrived to the server

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub const DATABASE_URL: &'static str = "app.db";

pub fn init_pool() -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
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
