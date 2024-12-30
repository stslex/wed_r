#[cfg(test)]
mod test {
    use teloxide::{
        dptree,
        types::{User, UserId},
    };
    use teloxide_tests::{MockBot, MockMessageText};

    use crate::{config::BotState, handlers::handler_tree};

    #[tokio::test]
    async fn test_start_route_success() {
        let name = "test first name";
        let user = User {
            id: UserId(12345),
            is_bot: false,
            first_name: name.to_owned(),
            last_name: Some("Last".to_owned()),
            username: Some("test username".to_owned()),
            language_code: None,
            is_premium: false,
            added_to_attachment_menu: false,
        };
        let message = MockMessageText::new().from(user).text("/start");

        let bot_state = BotState::new_test();
        let bot = MockBot::new(message, handler_tree());

        bot.dependencies(dptree::deps![bot_state]);

        // Sends the message as if it was from a user
        bot.dispatch().await;

        let responses = bot.get_responses();
        let message = responses
            .sent_messages
            .last()
            .expect("No sent messages were detected!");

        assert_eq!(
            message.text(),
            Some(format!("Welcome, {}! Choose an option:", name).as_str())
        );
    }

    #[tokio::test]
    async fn test_start_route_empty_username() {
        let name = "test first name";
        let username = "";
        let user = User {
            id: UserId(12345),
            is_bot: false,
            first_name: name.to_owned(),
            last_name: Some("Last".to_owned()),
            username: Some(username.to_owned()),
            language_code: None,
            is_premium: false,
            added_to_attachment_menu: false,
        };
        let message = MockMessageText::new().from(user).text("/start");

        let bot_state = BotState::new_test();
        let bot = MockBot::new(message, handler_tree());

        bot.dependencies(dptree::deps![bot_state]);

        // Sends the message as if it was from a user
        bot.dispatch().await;

        let responses = bot.get_responses();
        let message = responses
            .sent_messages
            .last()
            .expect("No sent messages were detected!");

        assert_eq!(message.text(), Some("Name or username is empty"));
    }

    #[tokio::test]
    async fn test_start_route_empty_name() {
        let name = "";
        let username = "test_username";
        let user = User {
            id: UserId(12345),
            is_bot: false,
            first_name: name.to_owned(),
            last_name: Some("Last".to_owned()),
            username: Some(username.to_owned()),
            language_code: None,
            is_premium: false,
            added_to_attachment_menu: false,
        };
        let message = MockMessageText::new().from(user).text("/start");

        let bot_state = BotState::new_test();
        let bot = MockBot::new(message, handler_tree());

        bot.dependencies(dptree::deps![bot_state]);

        // Sends the message as if it was from a user
        bot.dispatch().await;

        let responses = bot.get_responses();
        let message = responses
            .sent_messages
            .last()
            .expect("No sent messages were detected!");

        assert_eq!(message.text(), Some("Name or username is empty"));
    }

    #[tokio::test]
    async fn test_start_route_from_null_user() {
        let message = MockMessageText::new().text("/start");

        let bot_state = BotState::new_test();
        let bot = MockBot::new(message, handler_tree());

        bot.dependencies(dptree::deps![bot_state]);

        // Sends the message as if it was from a user
        bot.dispatch().await;

        let responses = bot.get_responses();
        let message = responses
            .sent_messages
            .last()
            .expect("No sent messages were detected!");

        assert_eq!(message.text(), Some("Name or username is empty"));
    }
}
