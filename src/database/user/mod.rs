use model::{UserCreateEntity, UserEntity, UserUpdateEntity};

use super::ErrorResponseDb;

mod database;
pub mod model;
mod tests;

pub trait UserDatabase {
    async fn get_user_by_username<'a>(
        self,
        username: &'a str,
    ) -> Result<UserEntity, ErrorResponseDb>;
    async fn get_user<'a>(self, uuid: &'a str) -> Result<UserEntity, ErrorResponseDb>;
    async fn update_user(self, user: UserUpdateEntity) -> Result<UserEntity, ErrorResponseDb>;
    async fn create_user(self, user: UserCreateEntity) -> Result<UserEntity, ErrorResponseDb>;
    async fn get_all_users<'a>(self) -> Result<Vec<UserEntity>, ErrorResponseDb>;
}
