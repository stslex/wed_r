use crate::{config::BotState, routes::model::ErrorResponseData};

use super::{
    admin::{is_admin, AdminRepository},
    model::{StartRequestModel, StartResponseModel},
    user::{model::UserRequestDataModel, UserRepository},
    StartRepository,
};

impl StartRepository for BotState {
    async fn start<'a>(
        &self,
        request: &StartRequestModel<'a>,
    ) -> Result<StartResponseModel, ErrorResponseData> {
        let is_user_admin = is_admin(request.username);
        match is_user_admin {
            true => self
                .start_admin(&request.into())
                .await
                .map(|admin| StartResponseModel {
                    messege: format!("Welcome, {}! Choose an option:", admin.name),
                    is_admin: is_user_admin,
                }),
            false => match self.get_user(&request.uuid).await {
                Ok(user) => self
                    .start_user(&UserRequestDataModel {
                        uuid: user.uuid,
                        username: &request.username,
                        name: &user.name,
                        is_active: true,
                        is_accepted: user.is_accepted,
                    })
                    .await
                    .map(|user| StartResponseModel {
                        messege: format!("Welcome, {}! Choose an option:", user.name),
                        is_admin: is_user_admin,
                    }),
                Err(e) => {
                    log::error!("Cannot get the user: {}", e);
                    Err(e)
                }
            },
        }
    }
}
