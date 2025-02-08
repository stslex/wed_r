#[cfg(test)]
mod tests {
    use crate::{
        config::BotState,
        repository::admin::{
            is_admin,
            model::{AdminRequestModel, CreateUserRequestModel},
            AdminRepository,
        },
    };

    #[tokio::test]
    async fn create_user() {
        let state = BotState::new_test();
        let user_create_model = CreateUserRequestModel {
            username: "test_username",
            name: "test_name",
        };

        let user_created = state.create_user(&user_create_model).await.unwrap();

        assert_eq!(user_created.name, user_create_model.name);
        assert_eq!(user_created.username, user_create_model.username);
        assert!(!user_created.uuid.to_string().is_empty());
    }

    #[tokio::test]
    async fn start_admin_user() {
        let state = BotState::new_test();
        let admin_create_model = AdminRequestModel {
            username: "admin_username",
            name: "admin_test_name",
            chat_id: &123,
        };

        let admin_created = state.start_admin(&admin_create_model).await.unwrap();

        assert_eq!(admin_created.name, admin_create_model.name);
        assert_eq!(admin_created.username, admin_create_model.username);
        assert_eq!(admin_created.chat_id, Some(*admin_create_model.chat_id));
        assert!(!admin_created.uuid.to_string().is_empty());
    }

    #[tokio::test]
    async fn is_admin_success() {
        let expected_username = "admin_username";
        std::env::set_var("ADMIN_USERNAME", expected_username);
        assert!(is_admin(expected_username));
    }

    #[tokio::test]
    async fn is_admin_not() {
        std::env::set_var("ADMIN_USERNAME", "admin_username");
        assert!(!is_admin("not_admin_username"));
    }
}
