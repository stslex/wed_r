use uuid::Uuid;

use crate::database::user::model::UserEntity;

#[derive(Clone, Copy)]
pub struct UserRequestDataModel<'a> {
    pub uuid: Uuid,
    pub username: &'a str,
    pub name: &'a str,
    pub is_accepted: bool,
    pub chat_id: &'a i64,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct UserResponseDataModel {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
    pub is_accepted: bool,
    pub chat_id: Option<i64>,
}

impl Into<UserResponseDataModel> for UserEntity {
    fn into(self) -> UserResponseDataModel {
        UserResponseDataModel {
            uuid: self.uuid,
            username: self.username,
            name: self.name,
            is_accepted: self.is_accepted,
            chat_id: self.chat_id,
        }
    }
}
