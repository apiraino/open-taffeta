use serde_derive::Deserialize;
use crate::auth::Auth;

#[derive(Deserialize, Debug)]
pub struct UserBaseResponse {
    pub id: i32,
    pub email: String,
    pub is_active: bool
}

#[derive(Deserialize, Debug)]
pub struct ResponseListUser {
    pub users: Vec<UserBaseResponse>
}

#[derive(Deserialize, Debug)]
pub struct ResponseUserDetail {
    pub user: UserBaseResponse
}

#[derive(Deserialize, Debug)]
pub struct ResponseLoginSignup {
    pub auth: Auth,
    pub is_active: bool
}

#[derive(Deserialize, Debug)]
pub struct ResponseSignupError {
    pub detail: String
}
