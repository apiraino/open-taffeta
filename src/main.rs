use std::env;

// extern crate env_logger;
// #[macro_use]
// extern crate log;

extern crate dotenv;

extern crate open_taffeta_lib;

fn main() {
    // reads the appropriate config file and sets env vars
    let deploy_env = env::var("DEPLOY_ENV").unwrap_or_else(|_| String::from("TEST"));
    let env_file = format!(".env_{}", deploy_env);
    dotenv::from_filename(env_file).ok();

    // TODO: fix logging
    // env_logger::init();

    let runner = open_taffeta_lib::runner().unwrap();
    // 🚀  Rocket has launched
    runner.launch();
}
