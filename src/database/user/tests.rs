#[cfg(test)]
mod tests {

    use crate::{
        config::database::pool::create_test_db_pool,
        database::{
            self,
            user::{model::UserCreateEntity, UserDatabase},
        },
    };

    #[tokio::test]
    async fn get_user_error() {
        let username = "test_username";
        let mut pool = create_test_db_pool().get().unwrap();
        let user_result = pool.get_user(username).await;

        assert_eq!(
            user_result.err().unwrap(),
            database::ErrorResponseDb::NotFound
        );
    }

    #[tokio::test]
    async fn create_user() {
        let username = "test_username";
        let name = "test_name";

        let entity_create = UserCreateEntity {
            username: username.to_string(),
            name: name.to_string(),
        };

        let mut pool = create_test_db_pool().get().unwrap();

        let user_result = pool.create_user(entity_create).await;

        let user = user_result.unwrap();
        assert_eq!(user.username, username);
        assert_eq!(user.name, name);
    }

    #[tokio::test]
    async fn create_user_error() {
        let entity_create = UserCreateEntity {
            username: "test_username".to_string(),
            name: "test_name".to_string(),
        };

        let mut pool = create_test_db_pool().get().unwrap();

        let user_result = pool.create_user(entity_create.clone()).await;
        assert!(user_result.is_ok());

        let user_result = pool.create_user(entity_create).await;

        assert_eq!(
            user_result.err().unwrap(),
            database::ErrorResponseDb::Conflict
        );
    }

    #[tokio::test]
    async fn get_user() {
        let username = "test_username";
        let name = "test_name";

        let entity_create = UserCreateEntity {
            username: username.to_string(),
            name: name.to_string(),
        };

        let mut pool = create_test_db_pool().get().unwrap();

        let user_result = pool.create_user(entity_create).await;
        assert!(user_result.is_ok());

        let user_result = pool.get_user(username).await;
        let user = user_result.unwrap();

        assert_eq!(user.username, username);
        assert_eq!(user.name, name);
    }
}
