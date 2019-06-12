extern crate diesel;
extern crate open_taffeta_lib;

use crate::common::dbstate::diesel::sqlite::SqliteConnection;
// this re-exports `.eq` from `diesel::ExpressionMethods`
use crate::common::dbstate::diesel::prelude::*;

use std::env;

use open_taffeta_lib::auth::token::Auth;
use open_taffeta_lib::models::{RoleNew, User};
use open_taffeta_lib::schema::doors;
use open_taffeta_lib::schema::roles;
use open_taffeta_lib::schema::userauth;
use open_taffeta_lib::schema::users;

pub struct DbState {
    pub conn: SqliteConnection,
}

impl DbState {
    pub fn new() -> DbState {
        // https://gitter.im/diesel-rs/diesel?at=5bc784d064cfc273f9e1607b
        // SqliteConnection::establish(":memory:")
        let database_url = env::var("DATABASE_URL").unwrap();
        DbState { conn: SqliteConnection::establish(&database_url).unwrap() }
    }

    pub fn create_user(&self, email: &str, is_active: bool, role: &str) -> User {
        let mut test_user = User::default();
        test_user.email = String::from(email);
        let test_password = open_taffeta_lib::config::generate_password();
        test_user.password = test_password.clone();

        diesel::insert_into(users::table)
            .values((
                users::password.eq(&test_user.password),
                users::email.eq(&test_user.email),
                users::is_active.eq(is_active),
            ))
            .execute(&self.conn)
            .expect("Test user could not be created.");

        let user: User = users::table
            .filter(users::email.eq(&test_user.email))
            .first(&self.conn)
            .expect(&format!("error getting user with email {}", test_user.email));

        let role_data = RoleNew { name: role.to_owned(), user: Some(user.id) };
        open_taffeta_lib::db::add_role(&self.conn, role_data);
        user
    }

    pub fn create_auth(
        &self,
        user_id: i32,
        email: &str,
        expiry_date: chrono::NaiveDateTime,
    ) -> Option<Auth> {
        let mut auth = Auth::new(user_id, email);
        auth.exp = expiry_date;
        if let Ok(_) = diesel::insert_into(userauth::table).values(&auth).execute(&self.conn) {
            Some(auth)
        } else {
            None
        }
    }

    pub fn count_auth_token(&self, uid: i32) -> i64 {
        let auth_count = userauth::table
            .filter(userauth::user_id.eq(uid))
            .count()
            .get_result(&self.conn)
            .expect(&format!("error getting token count for user id {}", uid));
        auth_count
    }

    pub fn assert_empty_users(&self) {
        assert_eq!(
            0,
            users::table.count().get_result::<i64>(&self.conn).expect("Failed to get users count")
        );
    }

    pub fn assert_empty_doors(&self) {
        assert_eq!(
            0,
            doors::table.count().get_result::<i64>(&self.conn).expect("Failed to get count")
        );
    }

    pub fn clean_tables(&self) {
        // TODO: truncate (if supported)
        diesel::delete(users::table).execute(&self.conn).expect("Cannot delete users");
        diesel::delete(doors::table).execute(&self.conn).expect("Cannot delete doors");
        diesel::delete(userauth::table).execute(&self.conn).expect("Cannot delete userauth");
        diesel::delete(roles::table).execute(&self.conn).expect("Cannot delete roles");
    }
}

impl Drop for DbState {
    fn drop(&mut self) {
        self.clean_tables();
    }
}
