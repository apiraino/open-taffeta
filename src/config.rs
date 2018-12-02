// TODO: Load from env
// https://rocket.rs/v0.4/guide/configuration/

// openssl rand -base64 32
pub const SECRET : &'static str = "Bgi/R1Lrznre9MINinMAXSaIVCXKSE+efLFFkx6dfPQ=";

#[macro_export]
macro_rules! get_token_duration {
    () => {
        chrono::Utc::now() + chrono::Duration::days(60)
    };
}
