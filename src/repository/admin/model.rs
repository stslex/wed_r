use uuid::Uuid;

use crate::database::user::model::UserEntity;

pub struct AdminRequestModel<'a> {
    pub username: &'a str,
    pub name: &'a str,
}

#[derive(Debug)]
pub struct CreateUserRequestModel<'a> {
    pub username: &'a str,
    pub name: &'a str,
}

#[allow(dead_code)]
pub struct UserResponseModel {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
}

impl<'a> Into<CreateUserRequestModel<'a>> for &AdminRequestModel<'a> {
    fn into(self) -> CreateUserRequestModel<'a> {
        CreateUserRequestModel {
            username: self.username,
            name: self.name,
        }
    }
}

impl Into<UserResponseModel> for UserEntity {
    fn into(self) -> UserResponseModel {
        UserResponseModel {
            uuid: self.uuid,
            username: self.username,
            name: self.name,
        }
    }
}
