use std::env;
use rand::prelude::*;
use rand::distributions::Alphanumeric;

// pub const TOKEN_LIFETIME : i64 = 10_000_000_000;
// TODO: wrestle with lifetimes ...
// pub const CLIENT_TYPE_WEB : &str = "client-type-web";

#[macro_export]
macro_rules! get_token_duration {
    () => {
        // create a DateTime+UTC
        // get the UNIX timestamp
        // create a naive DateTime, we loose UTC
        chrono::NaiveDateTime::from_timestamp((chrono::Utc::now() + chrono::Duration::days(180)).timestamp(), 0)
    };
}

pub fn get_secret() -> String {
    env::var("SECRET").expect("SECRET env var")
}

pub fn generate_password() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect::<String>()
}
