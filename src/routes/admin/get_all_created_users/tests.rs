#[cfg(test)]
mod tests {
    use teloxide::{
        dispatching::dialogue::InMemStorage,
        dptree,
        types::{User, UserId},
    };
    use teloxide_tests::{MockBot, MockMessageText};

    use crate::{
        config::{BotState, CreateUserState},
        handlers::handler_tree,
        repository::admin::{model::CreateUserRequestModel, AdminRepository},
    };

    #[tokio::test]
    async fn admin_route_get_all_created_success() {
        std::env::set_var("ADMIN_USERNAME", "test_username");

        let name = "test first name";
        let user = User {
            id: UserId(12345),
            is_bot: false,
            first_name: name.to_owned(),
            last_name: Some("Last".to_owned()),
            username: Some("test_username".to_owned()),
            language_code: None,
            is_premium: false,
            added_to_attachment_menu: false,
        };
        let message = MockMessageText::new().from(user).text("/createdusers");

        let bot_state = BotState::new_test();
        let bot_user_state = InMemStorage::<CreateUserState>::new();

        let created_user_model_1 = CreateUserRequestModel {
            username: "test_username_1",
            name: "test first name_1",
        };
        let created_user_model_2 = CreateUserRequestModel {
            username: "test_username_2",
            name: "test first name_2",
        };

        let expected_user_1 = bot_state.create_user(&created_user_model_1).await.unwrap();
        let expected_user_2 = bot_state.create_user(&created_user_model_2).await.unwrap();

        let bot = MockBot::new(message, handler_tree());

        bot.dependencies(dptree::deps![bot_state, bot_user_state]);

        // Sends the message as if it was from a user
        bot.dispatch().await;

        "Index | username | name | is active | is accepted |\nAll users:\n0 | @test_username_1 | test first name_1 | false | false |\n1 | @test_username_2 | test first name_2 | false | false |";
        let expected_message = format!(
            "Index | username | name | is active | is accepted |\nAll users:\n0 | @{} | {} | {} | {} |\n1 | @{} | {} | {} | {} |",
            expected_user_1.username,
            expected_user_1.name,
            expected_user_1.chat_id.is_some(),
            expected_user_1.is_accepted,
            expected_user_2.username,
            expected_user_2.name,
            expected_user_2.chat_id.is_some(),
            expected_user_2.is_accepted
        );

        let responses = bot.get_responses();
        let message = responses
            .sent_messages
            .last()
            .expect("No sent messages were detected!");

        assert_eq!(message.text(), Some(expected_message.as_str()));
    }
}
