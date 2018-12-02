#![feature(rust_2018_preview)]
// workaround this: https://github.com/rust-lang/rust/issues/50504#issuecomment-412341631
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

#[macro_use]
extern crate serde_derive;

extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;

extern crate validator;

extern crate chrono;

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
                routes::all::tester,
                routes::all::tester_2,
                routes::all::tester_3,
                routes::doors::create_door,
            ],
        ).manage(pool)
        .catch(catchers![
            // returns a 404 for URLs not mapped
            routes::all::not_found,
            routes::all::not_authorized,
            // routes::all::bad_request
        ]);

    Ok(rocket)
}
