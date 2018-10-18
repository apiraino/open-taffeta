extern crate diesel;
extern crate open_taffeta_lib;

use common::dbstate::diesel::sqlite::SqliteConnection;
// this re-exports `.eq` from `diesel::ExpressionMethods`
use common::dbstate::diesel::prelude::*;
use common::generate_password;

use std::env;

use open_taffeta_lib::models::User;
use open_taffeta_lib::schema::users::dsl::*;

pub struct DbState {
    conn: SqliteConnection,
}

impl DbState {

    pub fn new() -> DbState {
        // https://gitter.im/diesel-rs/diesel?at=5bc784d064cfc273f9e1607b
        // SqliteConnection::establish(":memory:")
        let database_url = env::var("DATABASE_URL").unwrap();
        DbState { conn: SqliteConnection::establish(&database_url).unwrap() }
    }

    // warning: "email" will collide with "open_taffeta_lib::schema::users::email" (duh)
    pub fn create_user(&self, email_fld: &str) -> (User, String) {
        let mut test_user = User::default();
        test_user.username = String::from("john");
        test_user.email = String::from(email_fld);
        let test_password = generate_password();
        test_user.password = test_password.clone();

        diesel::insert_into(users)
            .values((
                username.eq(&test_user.username),
                password.eq(&test_user.password),
                email.eq(&test_user.email),
            )).execute(&self.conn)
            .expect("Test user could not be created.");

        let user: User = users
            .filter(username.eq(&test_user.username))
            .first(&self.conn)
            .expect(&format!(
                "error getting user with username {}",
                test_user.username
            ));

        (user, test_password)
    }

    pub fn clean_db(&self) {
        // TODO: truncate (if supported)
        diesel::delete(users).execute(&self.conn)
            .expect("Cannot delete users");
    }

}

impl Drop for DbState {
    fn drop(&mut self) {
        self.clean_db();
    }
}
