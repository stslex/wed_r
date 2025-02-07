use crate::{
    config::{database::DataPool, BotState},
    database::user::{model::UserUpdateEntity, UserDatabase},
    routes::model::ErrorResponseData,
};

use super::{
    model::{UserRequestDataModel, UserResponseDataModel},
    UserRepository,
};

impl UserRepository for BotState {
    async fn get_user<'a>(
        &self,
        uuid: &'a str,
    ) -> Result<UserResponseDataModel, ErrorResponseData> {
        let mut pool = self.clone().safe_get()?;
        pool.get_user(uuid)
            .await
            .map(|user| user.into())
            .map_err(|err| err.into())
    }

    async fn start_user<'a>(
        &self,
        user: &UserRequestDataModel<'a>,
    ) -> Result<UserResponseDataModel, ErrorResponseData> {
        let mut pool = self.clone().safe_get()?;
        if user.username.is_empty() {
            return Err(ErrorResponseData::InternalServerError);
        }
        pool.update_user(UserUpdateEntity {
            uuid: user.uuid,
            username: user.username.to_owned(),
            name: user.name.to_owned(),
            is_active: user.is_active,
            is_accepted: user.is_accepted,
        })
        .await
        .map(|user| user.into())
        .map_err(|err| {
            println!("error {:?}", err);
            err.into()
        })
    }
}
