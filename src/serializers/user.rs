use crate::auth::token::Auth;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserBaseResponse {
    pub id: i32,
    pub email: String,
    pub is_active: bool,
    pub role: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseListUser {
    pub users: Vec<UserBaseResponse>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseUserDetail {
    pub user: UserBaseResponse,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseLoginSignup {
    pub auth: Auth,
    pub user: UserBaseResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserEdit<'a> {
    pub email: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub status: String,
    pub detail: String,
}
