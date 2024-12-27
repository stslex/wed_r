use model::{UserDbError, UserEntity};
use uuid::Uuid;

mod database;
pub mod model;

pub trait UserDatabase {
    async fn get_user(&self, uuid: Uuid) -> Result<UserEntity, UserDbError>;
}
