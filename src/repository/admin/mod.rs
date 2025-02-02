use model::{AdminRequestModel, CreateUserRequestModel, UserResponseModel};
use uuid::Uuid;

use crate::routes::model::ErrorResponseData;

use super::model::StartRequestModel;

pub mod model;
mod repository;
mod tests;

pub trait AdminRepository {
    #[allow(dead_code)]
    async fn create_user<'a>(
        &self,
        request: &CreateUserRequestModel<'a>,
    ) -> Result<UserResponseModel, ErrorResponseData>;
    async fn get_all_users<'a>(
        &self,
        username: &'a str,
    ) -> Result<Vec<UserResponseModel>, ErrorResponseData>;
    async fn start_admin<'a>(
        &self,
        request: &AdminRequestModel<'a>,
    ) -> Result<UserResponseModel, ErrorResponseData>;
    async fn remove_user<'a>(&self, uuid: &'a Uuid) -> Result<(), ErrorResponseData>;
}

impl<'a> Into<AdminRequestModel<'a>> for &StartRequestModel<'a> {
    fn into(self) -> AdminRequestModel<'a> {
        AdminRequestModel {
            username: self.username,
            name: self.name,
        }
    }
}

pub fn is_admin<'a>(username: &'a str) -> bool {
    match std::env::var("ADMIN_USERNAME") {
        Ok(env_username) => username == env_username,
        Err(e) => {
            log::error!("Cannot get the ADMIN_USERNAME env variable: {}", e);
            false
        }
    }
}
