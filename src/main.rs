use std::env;

// extern crate env_logger;
// #[macro_use]
// extern crate log;

use rocket::config::Environment;

extern crate dotenv;

extern crate open_taffeta_lib;

fn main() {
    // Load env vars
    let deploy_env = env::var("ROCKET_ENV").unwrap_or_else(|_| String::from("dev"));
    let env_file = format!(".env_{}", deploy_env);
    dotenv::from_filename(env_file).ok();

    let env = match deploy_env.as_ref() {
        "prod" => Environment::Production,
        "stage" => Environment::Staging,
        "dev" => Environment::Development,
        _ => Environment::Development
    };

    // TODO: fix logging
    // env_logger::init();

    let runner = open_taffeta_lib::runner(env).unwrap();
    // 🚀  Rocket has launched
    runner.launch();
}
