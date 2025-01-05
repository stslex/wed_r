use crate::config::BotState;

use super::{
    admin::{is_admin, AdminRepository},
    model::{StartRequestModel, StartResponseModel},
    user::{model::UserRequestDataModel, UserRepository},
    StartRepository,
};

impl StartRepository for BotState {
    async fn start<'a>(&self, request: &StartRequestModel<'a>) -> StartResponseModel {
        let is_user_admin = is_admin(request.username);
        let name = match is_user_admin {
            true => match self.start_admin(&request.into()).await {
                Ok(admin) => admin.name,
                Err(e) => {
                    log::error!("Cannot get the admin: {}", e);
                    return StartResponseModel {
                        messege: "initiate admin user failed, please write to get extra help"
                            .to_owned(),
                        is_admin: false,
                    };
                }
            },
            false => match self.get_user(&request.uuid).await {
                Ok(user) => {
                    self.start_user(&UserRequestDataModel {
                        uuid: user.uuid,
                        username: &request.username,
                        name: &user.name,
                    })
                    .await
                    .unwrap();
                    user.name
                }
                Err(e) => {
                    log::error!("Cannot get the user: {}", e);
                    return StartResponseModel {
                        messege: "initiate user failed, please write to get extra help".to_owned(),
                        is_admin: false,
                    };
                }
            },
        };

        StartResponseModel {
            messege: format!("Welcome, {}! Choose an option:", name),
            is_admin: is_user_admin,
        }
    }
}
