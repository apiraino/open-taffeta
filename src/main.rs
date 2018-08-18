#![feature(rust_2018_preview)]

// extern crate env_logger;
// #[macro_use]
// extern crate log;

extern crate dotenv;
use dotenv::dotenv;

extern crate open_taffeta_lib;

fn main() {
    // sets env vars based on the `.env` file
    dotenv().ok();

    // TODO: fix logging
    // env_logger::init();

    let runner = open_taffeta_lib::runner().unwrap();
    // 🚀  Rocket has launched
    runner.launch();
}
