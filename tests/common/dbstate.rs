extern crate diesel;
extern crate open_taffeta_lib;

use crate::common::dbstate::diesel::sqlite::SqliteConnection;
// this re-exports `.eq` from `diesel::ExpressionMethods`
use crate::common::dbstate::diesel::prelude::*;
use crate::common::generate_password;

use std::env;

use open_taffeta_lib::models::User;
use open_taffeta_lib::schema::users::dsl::*;
use open_taffeta_lib::schema::doors::dsl::*;

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

    // warning: "email" param colliding with fields in "open_taffeta_lib::schema::users::*" (duh)
    pub fn create_user(&self, email_fld: &str, is_active: bool) -> (User, String) {
        let mut test_user = User::default();
        test_user.email = String::from(email_fld);
        let test_password = generate_password();
        test_user.password = test_password.clone();

        diesel::insert_into(users)
            .values((
                password.eq(&test_user.password),
                email.eq(&test_user.email),
                active.eq(is_active)
            )).execute(&self.conn)
            .expect("Test user could not be created.");

        let user: User = users
            .filter(email.eq(&test_user.email))
            .first(&self.conn)
            .expect(&format!(
                "error getting user with email {}",
                test_user.email
            ));

        (user, test_password)
    }

    pub fn assert_empty_users(&self) {
        assert_eq!(0, users.count().execute(&self.conn).unwrap());
    }

    pub fn assert_empty_doors(&self) {
        assert_eq!(0, doors.count().execute(&self.conn).unwrap());
    }

    pub fn clean_tables(&self) {
        // TODO: truncate (if supported)
        diesel::delete(users).execute(&self.conn)
            .expect("Cannot delete users");
        diesel::delete(doors).execute(&self.conn)
            .expect("Cannot delete doors");
    }
}

impl Drop for DbState {
    fn drop(&mut self) {
        self.clean_tables();
    }
}
