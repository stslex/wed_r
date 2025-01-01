#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use crate::{
        config::database::pool::create_test_db_pool,
        database::{
            user::{
                model::{UserCreateEntity, UserUpdateEntity},
                UserDatabase,
            },
            ErrorResponseDb,
        },
    };

    #[tokio::test]
    async fn get_user_error() {
        let mut pool = create_test_db_pool().get().unwrap();
        let user_result = pool.get_user("test_username").await;

        assert_eq!(user_result.err().unwrap(), ErrorResponseDb::NotFound);
    }

    #[tokio::test]
    async fn create_user() {
        let entity_create = UserCreateEntity {
            username: "test_username".to_owned(),
            name: "test_name".to_owned(),
        };

        let mut pool = create_test_db_pool().get().unwrap();

        let user_result = pool.create_user(entity_create.clone()).await;

        let user = user_result.unwrap();
        assert_eq!(user.username, entity_create.username);
        assert_eq!(user.name, entity_create.name);
    }

    #[tokio::test]
    async fn create_user_error() {
        let entity_create = UserCreateEntity {
            username: "test_username".to_owned(),
            name: "test_name".to_owned(),
        };

        let mut pool = create_test_db_pool().get().unwrap();

        let user_result = pool.create_user(entity_create.clone()).await;
        assert!(user_result.is_ok());

        let user_result = pool.create_user(entity_create).await;

        assert_eq!(user_result.err().unwrap(), ErrorResponseDb::Conflict);
    }

    #[tokio::test]
    async fn get_user() {
        let entity_create = UserCreateEntity {
            username: "test_username".to_owned(),
            name: "test_name".to_owned(),
        };

        let mut pool = create_test_db_pool().get().unwrap();

        let user_result = pool.create_user(entity_create.clone()).await;
        assert!(user_result.is_ok());

        let user_result = pool.get_user(&entity_create.username).await;
        let user = user_result.unwrap();

        assert_eq!(user.username, entity_create.username);
        assert_eq!(user.name, entity_create.name);
    }

    #[tokio::test]
    async fn update_user() {
        let entity_create = UserCreateEntity {
            username: "test_username".to_owned(),
            name: "test_name".to_owned(),
        };

        let mut pool = create_test_db_pool().get().unwrap();

        let user_result = pool.create_user(entity_create).await;
        assert!(user_result.is_ok());

        let update_user = UserUpdateEntity {
            username: "new_username".to_owned(),
            name: "new_name".to_owned(),
            uuid: user_result.unwrap().uuid,
        };
        let update_result = pool.update_user(update_user.clone()).await.unwrap();

        assert_eq!(update_user.username, update_result.username);
        assert_eq!(update_user.name, update_result.name);
        assert_eq!(update_user.uuid, update_result.uuid);
    }

    #[tokio::test]
    async fn update_user_error_not_found() {
        let mut pool = create_test_db_pool().get().unwrap();

        let update_user = UserUpdateEntity {
            username: "new_username".to_owned(),
            name: "new_name".to_owned(),
            uuid: Uuid::new_v4(),
        };
        let update_result = pool.update_user(update_user.clone()).await.err().unwrap();

        assert_eq!(update_result, ErrorResponseDb::NotFound);
    }
}
