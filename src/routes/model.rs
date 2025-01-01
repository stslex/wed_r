use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ErrorResponseData {
    UserAlreadyExists,
    UserNotFound,
    InternalServerError,
    BlockingError,
}

impl fmt::Display for ErrorResponseData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ErrorResponseData::UserAlreadyExists => "User already exists",
            ErrorResponseData::UserNotFound => "User not found",
            ErrorResponseData::InternalServerError => "Internal server error",
            ErrorResponseData::BlockingError => "Blocking error",
        };
        write!(f, "{}", message)
    }
}
