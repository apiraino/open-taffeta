use dotenv;
use env_logger;
use rocket::config::Environment;
use std::env;

use open_taffeta_lib;

fn main() {
    // Load env vars
    let deploy_env = env::var("ROCKET_ENV").unwrap_or_else(|_| String::from("dev"));
    let env_file = format!(".env_{}", deploy_env);
    dotenv::from_filename(env_file).ok();

    let env = match deploy_env.as_ref() {
        "prod" => Environment::Production,
        "dev" => Environment::Development,
        _ => Environment::Development,
    };

    env_logger::init();

    let runner = open_taffeta_lib::runner(env).unwrap();
    // 🚀  Rocket has launched
    runner.launch();
}
