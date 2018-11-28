// #[macro_export]

// use chrono::{Duration, Utc, DateTime};

pub const SECRET : &'static str = "?qf3PjT9vrui`U:)i|@g";

// pub const TOKEN_DURATION : DateTime<Utc> = Utc::now() + Duration::days(60);
macro_rules! get_token_duration {
    () => { Utc::now() + Duration::days(60) }
}
