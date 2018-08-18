#![feature(rust_2018_preview)]
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

#[macro_use]
extern crate dotenv_codegen;

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
