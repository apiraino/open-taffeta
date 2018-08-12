#[allow(proc_macro_derive_resolution_fallback)]
use diesel::prelude::*;
use rocket_contrib::{Json, Value};

use crate::models::User;

use crate::db;

#[get("/user", format = "application/json")]
fn get_user(conn: db::Conn) -> Json<Value> {
    use crate::schema::users::dsl::users as all_users;
    // use schema::users::dsl::*;
    let rs = all_users
        .load::<User>(&*conn)
        .expect("error retrieving users");
    Json(json!(&rs))
}

#[get("/")]
fn get_blurb() -> &'static str {
    "Welcome!"
}
