use crate::domain::error::ApiError;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorPresenter {
    pub code: u16,
    pub error: String,
    pub message: String,
}

#[derive(Error, Debug, Display)]
#[display(fmt = "{:?}", error)]
pub struct ErrorReponse {
    status_code: StatusCode,
    error: String,
}

impl ResponseError for ErrorReponse {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorPresenter {
            code: status_code.as_u16(),
            message: status_code.to_string(),
            error: self.error.clone(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl ErrorReponse {
    pub fn map_io_error(e: ApiError) -> ErrorReponse {
        match e.get_error_code() {
            400 => ErrorReponse {
                status_code: StatusCode::BAD_REQUEST,
                error: e.get_error_message(),
            },
            401 => ErrorReponse {
                status_code: StatusCode::UNAUTHORIZED,
                error: e.get_error_message(),
            },
            403 => ErrorReponse {
                status_code: StatusCode::FORBIDDEN,
                error: e.get_error_message(),
            },
            _ => ErrorReponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: String::from("Error: an unknown error occured"),
            },
        }
    }
}
