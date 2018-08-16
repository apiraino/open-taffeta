#[allow(proc_macro_derive_resolution_fallback)]
use diesel::prelude::*;

use rocket_contrib::{Json, Value};

use crate::models::User;

use crate::db;

// SQL schema with same <id> fields causes:
//     `id` is ambiguous [E0659]
//use crate::schema::users::dsl::users as all_users;

// use crate::schema::users::dsl::users;

use crate::schema::users::dsl::*;

#[get("/users", format = "application/json")]
fn get_users(conn: db::Conn) -> Json<Value> {
    let users_rs = users.load::<User>(&*conn).expect("error retrieving users");
    Json(json!({ "users": users_rs }))
}

#[get("/user/<user_id>", format = "application/json")]
fn get_user(conn: db::Conn, user_id: i32) -> Json<Value> {
    let user: Vec<User> = users
        .filter(active.eq(true))
        .filter(id.eq(user_id))
        .load(&*conn)
        .expect(&format!("error retrieving user id={}", user_id));
    Json(json!({ "user": user }))
}
