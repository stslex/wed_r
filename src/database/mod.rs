use std::fmt::Display;

mod model;
pub mod user;

pub enum ErrorResponseDb {
    InternalServerError,
    NotFound,
    Conflict,
}

impl Display for ErrorResponseDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorResponseDb::InternalServerError => write!(f, "Internal server error"),
            ErrorResponseDb::NotFound => write!(f, "Not found"),
            ErrorResponseDb::Conflict => write!(f, "Conflict"),
        }
    }
}
