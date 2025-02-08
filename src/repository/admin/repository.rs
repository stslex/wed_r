use uuid::Uuid;

use crate::{
    config::{database::DataPool, BotState},
    database::{
        user::{model::UserCreateEntity, UserDatabase},
        ErrorResponseDb,
    },
    routes::model::ErrorResponseData,
};

use super::{
    is_admin,
    model::{AdminRequestModel, CreateUserRequestModel, UserResponseModel},
    AdminRepository,
};

impl AdminRepository for BotState {
    async fn get_all_users<'a>(
        &self,
        username: &'a str,
    ) -> Result<Vec<UserResponseModel>, ErrorResponseData> {
        if !is_admin(username) {
            return Err(ErrorResponseData::NoPermission);
        }
        let mut pool = self.clone().safe_get()?;
        match pool.get_all_users().await {
            Ok(users) => Ok(users.into_iter().map(|user| user.into()).collect()),
            Err(e) => {
                log::error!("Cannot get all users: {}", e);
                Err(ErrorResponseData::InternalServerError)
            }
        }
    }

    async fn start_admin<'a>(
        &self,
        request: &AdminRequestModel<'a>,
    ) -> Result<UserResponseModel, ErrorResponseData> {
        let mut pool = self.clone().safe_get()?;
        match pool.get_user_by_username(request.username).await {
            Ok(user) => Ok(user.into()),
            Err(e) => match e {
                ErrorResponseDb::NotFound => pool
                    .create_user(UserCreateEntity {
                        username: request.username.to_owned(),
                        name: request.name.to_owned(),
                        chat_id: Some(*request.chat_id),
                        is_accepted: false,
                    })
                    .await
                    .map(|user| user.into())
                    .map_err(|err| err.into()),
                e => {
                    log::error!("Cannot get the admin: {}", e);
                    Err(e.into())
                }
            },
        }
    }

    async fn create_user<'a>(
        &self,
        request: &CreateUserRequestModel<'a>,
    ) -> Result<UserResponseModel, ErrorResponseData> {
        let mut pool = self.clone().safe_get()?;
        pool.create_user(UserCreateEntity {
            username: request.username.to_owned(),
            name: request.name.to_owned(),
            is_accepted: false,
            chat_id: None,
        })
        .await
        .map(|user| user.into())
        .map_err(|err| err.into())
    }

    async fn remove_user<'a>(&self, uuid: &'a Uuid) -> Result<(), ErrorResponseData> {
        let mut pool = self.clone().safe_get()?;
        pool.remove_user(uuid).await.map_err(|err| err.into())
    }
}
