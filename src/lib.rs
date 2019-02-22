// workaround this: https://github.com/rust-lang/rust/issues/50504#issuecomment-412341631
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin)]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

// ensure macros are imported before
// any modules that might use them
#[macro_use]
pub mod config;
pub mod crypto;

mod db;
// TODO: this is wrong, used only for tests
pub mod models;
pub mod responses;
mod routes;
// TODO: pub here is wrong, used only for tests
pub mod schema;
mod auth;

use rocket::config::{Config, Environment};

pub fn runner(env: Environment) -> Result<rocket::Rocket, String> {
    let pool = db::init_pool();

    // default: localhost:8000
    let config = Config::build(env)
        .address("0.0.0.0")
        .tls("certs/localhost.pem", "certs/localhost-key.pem")
        .port(8080)
        .finalize().unwrap();

    let rocket = rocket::custom(config)
        // mount the routes
        .mount(
            "/",
            // plug the DB connection pool
            routes![
                routes::all::get_index,
                routes::users::get_users,
                routes::users::get_user,
                routes::users::signup_user,
                routes::doors::create_door,
                routes::doors::get_doors,
                routes::doors::get_door,
                routes::doors::delete_door,
                routes::doors::buzz_door
            ],
        ).manage(pool)
        .register(catchers![
            // returns a 404 for URLs not mapped
            routes::all::not_found,
            routes::all::not_authorized,
            // routes::all::bad_request
        ]);

    Ok(rocket)
}
