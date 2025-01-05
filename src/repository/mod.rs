use model::{StartRequestModel, StartResponseModel};

use crate::routes::model::ErrorResponseData;

pub mod admin;
pub mod model;
mod respository;
mod tests;
mod user;

pub trait StartRepository {
    async fn start<'a>(
        &self,
        request: &StartRequestModel<'a>,
    ) -> Result<StartResponseModel, ErrorResponseData>;
}
