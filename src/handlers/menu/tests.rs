#[cfg(test)]
mod test {
    use teloxide::{
        dptree,
        types::{User, UserId},
    };
    use teloxide_tests::{MockBot, MockMessageText};

    use crate::{config::BotState, handlers::handler_tree};

    #[tokio::test]
    async fn test_start_route() {
        let name = "First Name";
        let user = User {
            id: UserId(12345),
            is_bot: false,
            first_name: name.to_owned(),
            last_name: Some("Last".to_owned()),
            username: Some("username".to_owned()),
            language_code: None,
            is_premium: false,
            added_to_attachment_menu: false,
        };
        let message = MockMessageText::new().from(user).text("/start");

        let bot = MockBot::new(message, handler_tree());
        bot.dependencies(dptree::deps![BotState::new_test()]);

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
}
