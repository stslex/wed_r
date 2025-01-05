use model::{UserRequestDataModel, UserResponseDataModel};

use crate::routes::model::ErrorResponseData;

pub mod model;
mod repository;
mod tests;

pub trait UserRepository {
    async fn get_user<'a>(&self, uuid: &'a str)
        -> Result<UserResponseDataModel, ErrorResponseData>;
    async fn start_user<'a>(
        &self,
        user: &UserRequestDataModel<'a>,
    ) -> Result<UserResponseDataModel, ErrorResponseData>;
}
