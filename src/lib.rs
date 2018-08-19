#![feature(rust_2018_preview)]
// workaround this: https://github.com/rust-lang/rust/issues/50504#issuecomment-412341631
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate serde_derive;

extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;

mod db;
mod models;
mod routes;
mod schema;

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
                routes::doors::create_door,
            ],
        ).manage(pool)
        // returns a 404 for URLs not mapped
        .catch(catchers![routes::all::not_found]);

    Ok(rocket)
}
