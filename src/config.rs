use rand::distributions::Alphanumeric;
use rand::prelude::*;
use std::env;

pub const MAX_AUTH_TOKEN: i64 = 5;
// TODO: wrestle with lifetimes ...
// pub const CLIENT_TYPE_WEB : &str = "client-type-web";
pub const COOKIE_NAME_AUTH_STATUS: &str = "open-taffeta-admin-auth";

#[macro_export]
macro_rules! get_token_duration {
    () => {
        // create a DateTime+UTC
        // get the UNIX timestamp
        // create a naive DateTime, we loose UTC
        chrono::NaiveDateTime::from_timestamp(
            (chrono::Utc::now() + chrono::Duration::days(180)).timestamp(),
            0,
        )
    };
}

#[macro_export]
macro_rules! get_now {
    () => {
        // create a DateTime+UTC
        // get the UNIX timestamp
        // create a naive DateTime, we loose UTC
        // add an hour because we're not on UTC yet
        chrono::NaiveDateTime::from_timestamp(
            (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
            0,
        )
    };
}

pub fn get_secret() -> String {
    env::var("SECRET").expect("Missing SECRET env var")
}

pub fn generate_password() -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(30).collect::<String>()
}
