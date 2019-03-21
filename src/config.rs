use std::env;

#[macro_export]
macro_rules! get_token_duration {
    () => {
        chrono::Utc::now() + chrono::Duration::days(60)
    };
}

pub fn get_secret() -> String {
    env::var("SECRET").expect("SECRET env var")
}
