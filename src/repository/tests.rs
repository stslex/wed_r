#[cfg(test)]
mod tests {

    use crate::{
        config::BotState,
        repository::{model::UserRequestDataModel, UserRepository},
    };

    // Test success create new user in database :users
    // and return UserResponseDataModel
    #[tokio::test]
    async fn create_or_get_user_create_success() {
        let state = BotState::new_test();
        let user_create_model = UserRequestDataModel {
            username: "test_username",
            name: "test_name",
        };

        let user_response = state.create_or_get_user(user_create_model).await.unwrap();

        assert_eq!(user_response.name, user_create_model.name);
        assert_eq!(user_response.username, user_create_model.username);
        assert!(!user_response.uuid.to_string().is_empty())
    }

    // Test success return user from database :users
    // When user is already exist here
    #[tokio::test]
    async fn create_or_get_user_get_success() {
        let state = BotState::new_test();
        let user_create_model = UserRequestDataModel {
            username: "test_username",
            name: "test_name",
        };

        let user_create_response = state.create_or_get_user(user_create_model.clone()).await;
        assert!(user_create_response.is_ok());

        let user_get_response = state
            .create_or_get_user(user_create_model.clone())
            .await
            .unwrap();

        assert_eq!(user_get_response.name, user_create_model.name);
        assert_eq!(user_get_response.username, user_create_model.username);
        assert!(!user_get_response.uuid.to_string().is_empty());
        assert_eq!(user_get_response.uuid, user_create_response.unwrap().uuid);
    }
}
