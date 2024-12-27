use uuid::Uuid;

pub struct UserEntity {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
}

pub enum UserDbError {
    NotFound,
    Conflict,
    InternalError,
}
