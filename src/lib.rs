// #![feature(rust_2018_preview)]
// workaround this: https://github.com/rust-lang/rust/issues/50504#issuecomment-412341631
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin)]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]

// Only used in auth.rs
#[macro_use] extern crate serde_json;

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

// ensure macros are imported before
// any modules that might use them
#[macro_use]
mod config;

mod db;
// TODO: this is wrong, used only for tests
pub mod models;
pub mod responses;
mod routes;
// TODO: pub here is wrong, used only for tests
pub mod schema;
mod auth;

pub fn runner() -> Result<rocket::Rocket, String> {
    let pool = db::init_pool();
    let rocket = rocket::ignite()
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
                routes::doors::get_door
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
