use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub enum CookieError {
    Expired,
    Invalid, // forged cookie??
    Missing
}

pub struct AdminCookie {}

impl<'a, 'r> FromRequest<'a, 'r> for AdminCookie {
    type Error = CookieError;
    fn from_request(request: &'a Request<'r>)
                    -> request::Outcome<AdminCookie, Self::Error> {
        let mut cookies = request.cookies();
        match cookies.get_private("auth_status") {
            Some(_) => {
                Outcome::Success(AdminCookie{})
            },
            None => {
                // have the next route manage this case
                Outcome::Forward(())
            },
        }
    }
}
