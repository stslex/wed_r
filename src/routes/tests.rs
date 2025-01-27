#[cfg(test)]
mod tests {

    use crate::routes::get_payload;

    #[tokio::test]
    async fn get_payload_success() {
        let message = teloxide_tests::MockMessageText::new()
            .text("/start test")
            .build();
        let payload = get_payload(&message).await;
        assert_eq!(payload, "test");
    }

    #[tokio::test]
    async fn get_payload_error() {
        let message = teloxide_tests::MockMessageText::new()
            .text("/start")
            .build();
        let payload = get_payload(&message).await;
        assert_eq!(payload, "test");
    }
}
