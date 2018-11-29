// extern crate chrono;
// use chrono::{Duration, Utc};

pub const SECRET : &'static str = "?qf3PjT9vrui`U:)i|@g";

// pub const TOKEN_DURATION : DateTime<Utc> = Utc::now() + Duration::days(60);
#[macro_export]
macro_rules! get_token_duration {
    () => {
        chrono::Utc::now() + chrono::Duration::days(60)
    };
}
