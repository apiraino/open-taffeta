use serde_derive::Deserialize;
use crate::models::Door;

#[derive(Deserialize, Debug)]
pub struct ResponseDoorCreated {
    pub door: Door
}
