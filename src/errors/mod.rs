use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::derive::{Display, Error};

use crate::structs::BaseResponse;

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display("Internal Server Error :)")]
    InternalError,

    #[display("Bad Request 0_0")]
    BadClientData,

    #[display("Timeout!")]
    Timeout,

    #[display("Email Taken UwU")]
    EmailTaken,

    #[display("Not Found OwO!")]
    NotFound,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        HttpResponse::build(code).json(BaseResponse::<Option<String>> {
            data: None,
            code: code.as_u16(),
            message: self.to_string(),
        })
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadClientData => StatusCode::BAD_REQUEST,
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT,
            Self::EmailTaken => StatusCode::CONFLICT,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
