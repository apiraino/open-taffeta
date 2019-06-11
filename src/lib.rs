// workaround this: https://github.com/rust-lang/rust/issues/50504#issuecomment-412341631
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin)]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

// ensure macros are imported before
// any modules that might use them
#[macro_use]
pub mod config;
pub mod crypto;
pub mod utils;

pub mod db;
// TODO: pub here is wrong, used only for tests
pub mod auth;
pub mod models;
pub mod responses;
pub mod routes;
pub mod schema;
pub mod serializers;

use rocket::config::Environment;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

pub fn runner(_env: Environment) -> Result<rocket::Rocket, String> {
    let pool = db::init_pool();

    // example custom config
    // default: localhost:8000
    // let config = Config::build(env)
    //     .address("0.0.0.0")
    //     .tls("certs/localhost.pem", "certs/localhost-key.pem")
    //     .port(8888)
    //     .finalize().unwrap();

    let rocket = rocket::ignite()
        // mount the routes
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("STATIC_ASSETS_DIR"), "/static")),
        )
        .mount(
            "/",
            // plug the DB connection pool
            routes![
                routes::all::get_index,
                routes::users::get_users,
                routes::users::get_user,
                routes::users::login_user,
                routes::users::signup_user,
                routes::users::edit_user,
                routes::doors::create_door,
                routes::doors::get_doors,
                routes::doors::get_door,
                routes::doors::delete_door,
                routes::doors::buzz_door,
                routes::admin::admin_panel,
                routes::admin::admin_panel_redirect,
                routes::admin::admin_panel_get_login,
                routes::admin::admin_panel_get_login_noauth,
                routes::admin::admin_panel_post_login,
                routes::admin::admin_panel_edit_user
            ],
        )
        .attach(Template::fairing())
        .manage(pool)
        .register(catchers![
            // returns a 404 for URLs not mapped
            routes::all::not_found,
            routes::all::not_authorized,
            routes::all::unprocessable_entity
        ]);

    Ok(rocket)
}
