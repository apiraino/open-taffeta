// TODO: Load from env
// https://rocket.rs/v0.4/guide/configuration/
use std::env;

// openssl rand -base64 32
pub const SECRET : &str = "Bgi/R1Lrznre9MINinMAXSaIVCXKSE+efLFFkx6dfPQ=";

#[macro_export]
macro_rules! get_token_duration {
    () => {
        chrono::Utc::now() + chrono::Duration::days(60)
    };
}

pub fn get_buzzer_url() -> String {
    env::var("BUZZER_URL").expect("Could not find BUZZER_URL in env")
}
