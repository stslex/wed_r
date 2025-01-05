use model::{StartRequestModel, StartResponseModel};

mod admin;
pub mod model;
mod respository;
mod tests;
mod user;

pub trait StartRepository {
    async fn start<'a>(&self, request: &StartRequestModel<'a>) -> StartResponseModel;
}
