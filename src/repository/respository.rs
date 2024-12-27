use log::error;

use crate::{
    config::{database::DataPool, BotState},
    database::{
        user::{model::UserCreateEntity, UserDatabase},
        ErrorResponseDb,
    },
    routes::model::ErrorResponseData,
};

use super::{
    model::{UserRequestDataModel, UserResponseDataModel},
    UserRepository,
};

impl UserRepository for BotState {
    async fn create_or_get_user<'a>(
        &self,
        user: UserRequestDataModel<'a>,
    ) -> Result<UserResponseDataModel, ErrorResponseData> {
        let mut pool = self.clone().safe_get()?;
        if user.username.is_empty() {
            return Err(ErrorResponseData::InternalServerError);
        }
        match pool.get_user(&user.username).await {
            Ok(user) => Ok(user),
            Err(ErrorResponseDb::NotFound) => {
                let user = UserCreateEntity {
                    username: user.username.to_owned(),
                    name: user.name.to_owned(),
                };
                pool.create_user(user).await
            }
            Err(e) => {
                error!("Failed to get user: {}", e);
                Err(e)
            }
        }
        .map(|user| UserResponseDataModel {
            uuid: user.uuid,
            username: user.username,
            name: user.name,
        })
        .map_err(|err| match err {
            ErrorResponseDb::InternalServerError => ErrorResponseData::InternalServerError,
            ErrorResponseDb::NotFound => ErrorResponseData::UserNotFound,
            ErrorResponseDb::Conflict => ErrorResponseData::UserAlreadyExists,
        })
    }
}
