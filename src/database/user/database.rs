use std::str::FromStr;

use diesel::{result::DatabaseErrorKind, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::error;
use uuid::Uuid;

use crate::{config::DbCon, database::ErrorResponseDb, schema::users};

use super::{
    model::{UserCreateEntity, UserEntity, UserUpdateEntity},
    UserDatabase,
};

impl UserDatabase for &mut DbCon {
    async fn get_all_users<'a>(self) -> Result<Vec<UserEntity>, ErrorResponseDb> {
        users::table.load::<UserEntity>(self).map_err(|err| {
            error!("Failed to get all users: {}", err);
            ErrorResponseDb::InternalServerError
        })
    }
    async fn get_user_by_username<'a>(
        self,
        username: &'a str,
    ) -> Result<UserEntity, ErrorResponseDb> {
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

    async fn get_user<'a>(self, uuid: &'a str) -> Result<UserEntity, ErrorResponseDb> {
        let uuid = Uuid::from_str(uuid).map_err(|_| {
            error!("Failed to parse uuid");
            ErrorResponseDb::UuidInvalid
        })?;
        users::table
            .filter(users::uuid.eq(uuid))
            .first::<UserEntity>(self)
            .map_err(|err| {
                error!("Failed to get user: {}", err);
                match err {
                    diesel::result::Error::NotFound => ErrorResponseDb::NotFound,
                    _ => ErrorResponseDb::InternalServerError,
                }
            })
    }

    async fn update_user(self, user: UserUpdateEntity) -> Result<UserEntity, ErrorResponseDb> {
        diesel::update(users::table)
            .set(&user)
            .get_result::<UserEntity>(self)
            .map_err(|err| {
                println!("UserEntity error {:?}", err);
                error!("Failed to update user: {}", err);
                match err {
                    diesel::result::Error::NotFound => ErrorResponseDb::NotFound,
                    _ => ErrorResponseDb::InternalServerError,
                }
            })
    }

    async fn create_user(self, user: UserCreateEntity) -> Result<UserEntity, ErrorResponseDb> {
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
