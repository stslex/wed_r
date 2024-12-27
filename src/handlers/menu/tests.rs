#[cfg(test)]
mod test {
    use teloxide::dptree;
    use teloxide_tests::{MockBot, MockMessageText};

    use crate::{config::get_test_bot_state, handlers::handler_tree};

    #[tokio::test]
    async fn test_start_route() {
        let message = MockMessageText::new().text("/start");

        let bot = MockBot::new(message, handler_tree());
        bot.dependencies(dptree::deps![get_test_bot_state()]);

        // Sends the message as if it was from a user
        bot.dispatch().await;

        let responses = bot.get_responses();
        let message = responses
            .sent_messages
            .last()
            .expect("No sent messages were detected!");
        assert_eq!(message.text(), Some("Welcome! Choose an option:"));
    }
}
