extern crate diesel;
extern crate open_taffeta_lib;

use common::dbstate::diesel::sqlite::SqliteConnection;
// this re-exports `.eq` from `diesel::ExpressionMethods`
use common::dbstate::diesel::prelude::*;
use std::env;

use open_taffeta_lib::models::User;
use open_taffeta_lib::schema::users::dsl::*;

#[derive(Default)]
pub struct DbState;

// warning "email" will collide with "open_taffeta_lib::schema::users::email" (duh)
fn create_user(email_fld: &str) -> (User, String) {
    let mut test_user = User::default();
    test_user.username = String::from("john");
    test_user.email = String::from(email_fld);
    // TODO: a nice to have
    // let test_password: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    // test_user.password = pbkdf2_simple(&test_password, 5000).unwrap();
    test_user.password = String::from("7654321");

    // let database = configure_database_from_env().unwrap();
    let database_url = env::var("DATABASE_URL").unwrap();
    let conn = SqliteConnection::establish(&database_url).unwrap();
    diesel::insert_into(users)
        .values((
            username.eq(&test_user.username),
            password.eq(&test_user.password),
            email.eq(&test_user.email),
        )).execute(&conn)
        .expect("Test user could not be created.");

    // FIXME: boo cloning boo
    (test_user.clone(), test_user.password)
}

fn clean_db() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let conn = SqliteConnection::establish(&database_url).unwrap();
    // TODO: truncate (if supported)
    diesel::delete(users).execute(&conn)
        .expect("Cannot delete users");
}

impl DbState {
    pub fn setup(self, email_fld: &str) -> (User, String) {
        create_user(email_fld)
    }
    pub fn setup2(email_fld: &str) -> (User, String) {
        create_user(email_fld)
    }
}

impl Drop for DbState {
    fn drop(&mut self) {
        clean_db();
    }
}
