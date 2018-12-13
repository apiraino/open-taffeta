// Verbatim copy from:
// https://github.com/svenstaro/rust-web-boilerplate/blob/11cca61526c62bf429fd1f8c5f5ce693ff85eee5/src/responses.rs

use diesel::result::Error as DieselError;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use std::convert::From;
use std::io::Cursor;

#[derive(Debug)]
pub struct APIResponse {
    data: JsonValue,
    status: Status,
}

impl APIResponse {
    /// Set the data of the `Response` to `data`.
    pub fn data(mut self, data: JsonValue) -> APIResponse {
        self.data = data;
        self
    }

    /// Convenience method to set `self.data` to `{"detail": detail}`.
    pub fn detail(mut self, detail: &str) -> APIResponse {
        self.data = json!({ "detail": detail });
        self
    }
}

impl From<DieselError> for APIResponse {
    fn from(_: DieselError) -> Self {
        internal_server_error()
    }
}

impl<'r> Responder<'r> for APIResponse {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let body = self.data;

        Response::build()
            .status(self.status)
            .sized_body(Cursor::new(body.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}

pub fn ok() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::Ok,
    }
}

pub fn created() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::Created,
    }
}

pub fn accepted() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::Accepted,
    }
}

pub fn no_content() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::NoContent,
    }
}

pub fn bad_request() -> APIResponse {
    APIResponse {
        data: json!({"message": "Bad Request"}),
        status: Status::BadRequest,
    }
}

pub fn unauthorized() -> APIResponse {
    APIResponse {
        data: json!({"message": "Unauthorized"}),
        status: Status::Unauthorized,
    }
}

pub fn forbidden() -> APIResponse {
    APIResponse {
        data: json!({"message": "Forbidden"}),
        status: Status::Forbidden,
    }
}

pub fn not_found() -> APIResponse {
    APIResponse {
        data: json!({"message": "Not Found"}),
        status: Status::NotFound,
    }
}

pub fn method_not_allowed() -> APIResponse {
    APIResponse {
        data: json!({"message": "Method Not Allowed"}),
        status: Status::MethodNotAllowed,
    }
}

pub fn conflict() -> APIResponse {
    APIResponse {
        data: json!({"message": "Conflict"}),
        status: Status::Conflict,
    }
}

pub fn unprocessable_entity(errors: Value) -> APIResponse {
    APIResponse {
        data: json!({ "message": errors }),
        status: Status::UnprocessableEntity,
    }
}

pub fn internal_server_error() -> APIResponse {
    APIResponse {
        data: json!({"message": "Internal Server Error"}),
        status: Status::InternalServerError,
    }
}

pub fn service_unavailable() -> APIResponse {
    APIResponse {
        data: json!({"message": "Service Unavailable"}),
        status: Status::ServiceUnavailable,
    }
}
