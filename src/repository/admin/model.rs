use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::user::model::UserEntity;

pub struct AdminRequestModel<'a> {
    pub username: &'a str,
    pub name: &'a str,
    pub chat_id: &'a i64,
}

#[derive(Debug)]
pub struct CreateUserRequestModel<'a> {
    pub username: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UserResponseModel {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
    pub is_accepted: bool,
    pub chat_id: Option<i64>,
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
            is_accepted: self.is_accepted,
            chat_id: self.chat_id,
        }
    }
}

impl Display for UserResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let open_chat_id = if let Some(id) = self.chat_id { id } else { -1 };

        write!(
            f,
            "User: {} - {} - {} - {} - {}",
            self.username,
            self.name,
            self.uuid.to_string(),
            self.is_accepted,
            open_chat_id
        )
    }
}
