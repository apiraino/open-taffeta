use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub enum CookieError {
    Expired,
    Invalid, // forged cookie??
    Missing
}

// here only to check cookies ...
pub struct AdminUser {}

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = CookieError;
    fn from_request(request: &'a Request<'r>)
                    -> request::Outcome<AdminUser, Self::Error> {
        let mut cookies = request.cookies();
        match cookies.get_private("auth_status") {
            Some(_) => {
                Outcome::Success(AdminUser{})
            },
            None => {
                // have the next route manage this case
                Outcome::Forward(())
            },
        }
    }
}
