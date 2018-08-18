#![feature(rust_2018_preview)]
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

// extern crate env_logger;
// #[macro_use]
// extern crate log;

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
#[macro_use]
extern crate validator_derive;

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

use dotenv::dotenv;
use rocket_contrib::{Json, Value};

#[macro_use]
extern crate dotenv_codegen;

#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    // sets env vars based on the `.env` file
    dotenv().ok();

    // TODO: fix logging
    // env_logger::init();

    let pool = db::init_pool();
    rocket::ignite()
        // mount the routes
        .mount(
            "/",
            // plug the DB connection pool
            routes![
                routes::all::get_index,
                routes::users::get_users,
                routes::users::get_user,
                routes::doors::create_door,
            ],
        ).manage(pool)
        // returns a 404 for URLs not mapped
        .catch(catchers![not_found])
        // 🚀  Rocket has launched
        .launch();
}
