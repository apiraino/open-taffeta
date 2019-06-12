use crate::models::Door;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseDoorCreated {
    pub door: Door,
}
