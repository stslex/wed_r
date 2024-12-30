use model::{UserCreateEntity, UserEntity};

use super::ErrorResponseDb;

mod database;
pub mod model;
mod tests;

pub trait UserDatabase {
    async fn get_user<'a>(self, username: &'a str) -> Result<UserEntity, ErrorResponseDb>;
    async fn create_user(self, user: UserCreateEntity) -> Result<UserEntity, ErrorResponseDb>;
}
