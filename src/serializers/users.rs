use serde_derive::{Deserialize, Serialize};
use crate::models::UserAuthFull;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserBaseResponse {
    pub id: i32,
    pub email: String,
    pub active: bool
}

#[derive(Deserialize, Debug)]
pub struct UserAuthResponse {
    user_id: i32,
    email: String,
    token: String
}

#[derive(Deserialize, Debug)]
pub struct ResponseLoginSignup {
    pub user: UserAuthFull
}

#[derive(Deserialize, Debug)]
pub struct ResponseListUser {
    pub users: Vec<UserBaseResponse>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUserDetail {
    pub user: UserBaseResponse
}

#[derive(Deserialize, Debug)]
pub struct ResponseSignupError {
    pub detail: String
}
