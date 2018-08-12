#![feature(rust_2018_preview)]
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate rocket;

extern crate rocket_cors;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

extern crate validator;

extern crate crypto;
extern crate dotenv;

extern crate chrono;
extern crate frank_jwt as jwt;

extern crate rand;
extern crate slug;

mod db;
mod models;
mod routes;
mod schema;

// use rocket::http::{ContentType, Status};
// use rocket::request::Request;
// use rocket::response::{Responder, Response};
use rocket_contrib::{Json, Value};
// use schema::users;

#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    let pool = db::init_pool();
    rocket::ignite()
        // mount the routes
        .mount(
            "/",
            routes![routes::all::get_blurb, routes::all::get_user], // plug the DB connection pool
        ).manage(pool)
        // returns a 404 for URLs not mapped
        .catch(catchers![not_found])
        // 🚀  Rocket has launched
        .launch();
}
