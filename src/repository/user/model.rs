use uuid::Uuid;

use crate::database::user::model::UserEntity;

#[derive(Clone, Copy)]
pub struct UserRequestDataModel<'a> {
    pub uuid: Uuid,
    pub username: &'a str,
    pub name: &'a str,
    pub is_active: bool,
    pub is_accepted: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct UserResponseDataModel {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
    pub is_active: bool,
    pub is_accepted: bool,
}

impl Into<UserResponseDataModel> for UserEntity {
    fn into(self) -> UserResponseDataModel {
        UserResponseDataModel {
            uuid: self.uuid,
            username: self.username,
            name: self.name,
            is_active: self.is_active,
            is_accepted: self.is_accepted,
        }
    }
}
