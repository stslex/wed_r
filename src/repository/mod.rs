use model::{UserRequestDataModel, UserResponseDataModel};

use crate::routes::model::ErrorResponseData;

pub mod model;
mod respository;

pub trait UserRepository {
    async fn create_or_get_user<'a>(
        &self,
        user: UserRequestDataModel<'a>,
    ) -> Result<UserResponseDataModel, ErrorResponseData>;
}
