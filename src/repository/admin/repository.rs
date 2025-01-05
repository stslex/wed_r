use crate::{
    config::{database::DataPool, BotState},
    database::{
        user::{model::UserCreateEntity, UserDatabase},
        ErrorResponseDb,
    },
    routes::model::ErrorResponseData,
};

use super::{
    model::{
        AdminRequestModel, AdminResponseModel, CreateUserRequestModel, CreateUserResponseModel,
    },
    AdminRepository,
};

impl AdminRepository for BotState {
    async fn start_admin<'a>(
        &self,
        request: &AdminRequestModel<'a>,
    ) -> Result<AdminResponseModel, ErrorResponseData> {
        let mut pool = self.clone().safe_get()?;
        match pool.get_user_by_username(request.username).await {
            Ok(user) => Ok(AdminResponseModel {
                uuid: user.uuid,
                username: user.username,
                name: user.name,
            }),
            Err(e) => match e {
                ErrorResponseDb::NotFound => self
                    .create_user(&request.into())
                    .await
                    .map(|user| user.into()),
                _ => {
                    log::error!("Cannot get the admin: {}", e);
                    Err(ErrorResponseData::InternalServerError)
                }
            },
        }
    }

    async fn create_user<'a>(
        &self,
        request: &CreateUserRequestModel<'a>,
    ) -> Result<CreateUserResponseModel, ErrorResponseData> {
        let mut pool = self.clone().safe_get()?;
        match pool
            .create_user(UserCreateEntity {
                username: request.username.to_owned(),
                name: request.name.to_owned(),
            })
            .await
        {
            Ok(user) => Ok(CreateUserResponseModel {
                uuid: user.uuid,
                username: user.username,
                name: user.name,
            }),
            Err(e) => {
                log::error!("Cannot create the user: {}", e);
                Err(ErrorResponseData::InternalServerError)
            }
        }
    }
}
