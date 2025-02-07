use std::fmt::Display;

use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UserResponseModel {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
    pub is_active: bool,
    pub is_accepted: bool,
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
            is_active: self.is_active,
            is_accepted: self.is_accepted,
        }
    }
}

impl Display for UserResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User: {} - {} - {} - {} - {}",
            self.username,
            self.name,
            self.uuid.to_string(),
            self.is_active,
            self.is_accepted
        )
    }
}
