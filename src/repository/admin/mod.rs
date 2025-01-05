use model::{
    AdminRequestModel, AdminResponseModel, CreateUserRequestModel, CreateUserResponseModel,
};

use crate::routes::model::ErrorResponseData;

use super::model::StartRequestModel;

pub mod model;
mod repository;
mod tests;

pub trait AdminRepository {
    async fn create_user<'a>(
        &self,
        request: &CreateUserRequestModel<'a>,
    ) -> Result<CreateUserResponseModel, ErrorResponseData>;
    async fn start_admin<'a>(
        &self,
        request: &AdminRequestModel<'a>,
    ) -> Result<AdminResponseModel, ErrorResponseData>;
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
