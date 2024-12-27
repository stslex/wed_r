use diesel::{result::DatabaseErrorKind, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::error;

use crate::{config::DbCon, database::ErrorResponseDb, schema::users};

use super::{model::UserEntity, UserDatabase};

impl UserDatabase for &mut DbCon {
    async fn get_user<'a>(
        self,
        username: &'a str,
    ) -> Result<super::model::UserEntity, ErrorResponseDb> {
        users::table
            .filter(users::username.eq(username))
            .first::<UserEntity>(self)
            .map_err(|err| {
                error!("Failed to get user by username: {}", err);
                match err {
                    diesel::result::Error::NotFound => ErrorResponseDb::NotFound,
                    _ => ErrorResponseDb::InternalServerError,
                }
            })
    }

    async fn create_user(
        self,
        user: super::model::UserCreateEntity,
    ) -> Result<super::model::UserEntity, ErrorResponseDb> {
        if self.get_user(&user.username).await.is_ok() {
            return Err(ErrorResponseDb::Conflict);
        }
        diesel::insert_into(users::table)
            .values(user)
            .get_result::<UserEntity>(self)
            .map_err(|err| {
                error!("Failed to create user: {}", err);
                match err {
                    diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        ErrorResponseDb::Conflict
                    }
                    _ => ErrorResponseDb::InternalServerError,
                }
            })
    }
}
