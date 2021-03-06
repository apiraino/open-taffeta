// this file maps the DB tables as Rust structures
// Meaning of derives:
// (De)Serialize => can be used to (de)serialize data in JSON
// Queryable => can be used to represent entities in result set generated by queries
// AsChangeSet + Indentifiable => for `.save_changes()`
// ... there are many more ...

use crate::auth::token::Auth;
use crate::schema::{roles, users};
use diesel::dsl::Select;
use serde_derive::{Deserialize, Serialize};

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Debug,
    Default,
    Insertable,
    AsChangeset,
    Identifiable,
    PartialEq,
)]
pub struct User {
    pub id: i32,
    pub password: String,
    pub email: String,
    pub is_active: bool,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Default)]
pub struct Door {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub buzzer_url: String,
    pub ring: bool,
    pub ring_ts: Option<i32>,
}

// TODO: maybe an enum will do
// pub enum RoleName {
//     ADMIN = 0,
//     USER,
// }

pub const ROLE_ADMIN: &str = "admin";
pub const ROLE_USER: &str = "user";

#[derive(Debug, Identifiable, Queryable, Serialize, Deserialize, AsChangeset)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub user: Option<i32>,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, AsChangeset)]
#[table_name = "roles"]
pub struct RoleNew {
    pub name: String,
    pub user: Option<i32>,
}

// type DieselResult<T> = Result<T, diesel::result::Error>;

type AllColumns = (users::id, users::email, users::is_active);

pub const ALL_COLUMNS: AllColumns = (users::id, users::email, users::is_active);

type All = Select<users::table, AllColumns>;

impl User {
    // generate tokens for signup + logins
    pub fn to_auth(&self) -> Auth {
        Auth::new(self.id, &self.email)
    }

    // TODO: make it work
    pub fn no_pwd_fld(&self) -> All {
        use crate::diesel::QueryDsl;
        users::table.select(ALL_COLUMNS)
    }

    // TODO: make it work, see: http://diesel.rs/guides/composing-applications/
    // pub fn with_role(conn: &diesel::SqliteConnection) ->
    //     DieselResult<Vec<(User, Role)>>
    // {
    //     use crate::diesel::RunQueryDsl;
    //     users::table
    //         .inner_join(roles::table)
    //         .select(roles::name)
    //         .load(conn)
    // }

    pub fn is_allowed(&self, req: &rocket::Request, role: &str, user_id: i32) -> bool {
        let route = req.route().expect("Could not unwrap route from request");
        if role == ROLE_USER {
            // Users cannot list users, doors
            // and access admin pages
            match route.uri.path() {
                "/users" | "/admin" | "/doors" => {
                    return false;
                }
                _ => {}
            };

            // Users cannot delete anything (a.t.m.)
            if route.method == rocket::http::Method::Delete {
                return false;
            }
            // Ensure users don't mess with other profiles
            if route.uri.path() == "/users/<user_id>" {
                let req_id = req.uri().segments().last().expect("Could not extract id from route");
                let req_id_int = req_id.parse::<i32>().unwrap();
                if req_id_int != user_id {
                    return false;
                }
            }
        }
        true
    }
}
