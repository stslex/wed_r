use crate::config::DbPool;

use super::UserDatabase;

impl UserDatabase for DbPool {
    async fn get_user(
        &self,
        uuid: uuid::Uuid,
    ) -> Result<super::model::UserEntity, super::model::UserDbError> {
        todo!()
    }
}
