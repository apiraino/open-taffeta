use crate::models::{Role, User};
use crate::serializers::user::UserBaseResponse;

pub fn attach_role_to_user(u: &User, r: &Role) -> UserBaseResponse {
    UserBaseResponse {
        id: u.id,
        email: String::from(&u.email),
        is_active: u.is_active,
        role: String::from(&r.name),
    }
}
