use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::config;

#[derive(Debug)]
pub enum CookieError {
    Expired,
    Invalid, // forged cookie??
    Missing,
}

pub struct AdminCookie {
    pub user_id: i32
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminCookie {
    type Error = CookieError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminCookie, Self::Error> {
        let mut cookies = request.cookies();
        match cookies.get_private(config::COOKIE_NAME_AUTH_STATUS) {
            Some(val) => {
                let (cookie_name, cookie_value) = val.name_value();
                let v : Vec<&str> = cookie_value.split(':').collect();
                let user_id = v[0].parse::<i32>().unwrap();
                Outcome::Success(AdminCookie {user_id})
            },
            None => {
                // have the next route manage this case
                Outcome::Forward(())
            }
        }
    }
}
