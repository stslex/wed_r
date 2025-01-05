use std::fmt::Display;

pub mod user;

#[derive(Debug, PartialEq)]
pub enum ErrorResponseDb {
    InternalServerError,
    UuidInvalid,
    NotFound,
    Conflict,
}

impl Display for ErrorResponseDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorResponseDb::InternalServerError => write!(f, "Internal server error"),
            ErrorResponseDb::UuidInvalid => write!(f, "Uuid invalid"),
            ErrorResponseDb::NotFound => write!(f, "Not found"),
            ErrorResponseDb::Conflict => write!(f, "Conflict"),
        }
    }
}
