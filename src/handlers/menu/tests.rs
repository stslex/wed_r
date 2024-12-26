#[cfg(test)]
mod test {
    use teloxide_tests::{MockBot, MockMessageText};

    use crate::handlers::handler_tree;

    #[tokio::test]
    async fn test_start_route() {
        let message = MockMessageText::new().text("/start");
        let bot = MockBot::new(message, handler_tree());
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
