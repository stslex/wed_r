#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use crate::{
        config::BotState,
        repository::{
            admin::{model::CreateUserRequestModel, AdminRepository},
            user::{model::UserRequestDataModel, UserRepository},
        },
        routes::model::ErrorResponseData,
    };

    // Test success create new user in database :users
    // and return UserResponseDataModel
    #[tokio::test]
    async fn start_user_failed() {
        let state = BotState::new_test();
        let user_create_model = UserRequestDataModel {
            uuid: Uuid::new_v4(),
            username: "test_username",
            name: "test_name",
            is_accepted: false,
            chat_id: &123,
        };

        let user_response = state.start_user(&user_create_model).await.err().unwrap();
        assert_eq!(user_response, ErrorResponseData::UserNotFound);
    }

    // Test success return user from database :users
    // When user is already exist here
    #[tokio::test]
    async fn start_user_success() {
        let state = BotState::new_test();
        let user_create_model = CreateUserRequestModel {
            username: "test_username",
            name: "test_name",
        };

        let user_created = state.create_user(&user_create_model).await.unwrap();

        let user_start_model = UserRequestDataModel {
            uuid: user_created.uuid,
            username: &user_created.username,
            name: &user_created.name,
            is_accepted: false,
            chat_id: &123,
        };

        let user_get_response = state.start_user(&user_start_model.clone()).await;

        let user_get_response = user_get_response.unwrap();
        assert_eq!(user_get_response.name, user_start_model.name);
        assert_eq!(user_get_response.username, user_start_model.username);
        assert!(!user_get_response.uuid.to_string().is_empty());
        assert_eq!(user_get_response.uuid, user_start_model.uuid);
        assert_eq!(user_get_response.chat_id, Some(*user_start_model.chat_id));
    }
}
