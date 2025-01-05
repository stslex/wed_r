use std::fmt;

use crate::database::ErrorResponseDb;

#[derive(Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum ErrorResponseData {
    UserAlreadyExists,
    UserNotFound,
    InternalServerError,
    BlockingError,
    UuidInvalid,
}

impl fmt::Display for ErrorResponseData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ErrorResponseData::UserAlreadyExists => "User already exists",
            ErrorResponseData::UserNotFound => "User not found",
            ErrorResponseData::InternalServerError => "Internal server error",
            ErrorResponseData::BlockingError => "Blocking error",
            ErrorResponseData::UuidInvalid => "Invalid UUID",
        };
        write!(f, "{}", message)
    }
}

impl Into<ErrorResponseData> for ErrorResponseDb {
    fn into(self) -> ErrorResponseData {
        match self {
            ErrorResponseDb::Conflict => ErrorResponseData::UserAlreadyExists,
            ErrorResponseDb::NotFound => ErrorResponseData::UserNotFound,
            ErrorResponseDb::InternalServerError => ErrorResponseData::InternalServerError,
            ErrorResponseDb::UuidInvalid => ErrorResponseData::UuidInvalid,
        }
    }
}
