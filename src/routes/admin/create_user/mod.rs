use crate::{config::BotState, repository::admin::is_admin};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ReplyParameters},
    Bot,
};

mod tests;

// - route create user
// - check if user is admin
// - return callback for insert first name
// - return callback for insert state / key (for group user)
// - create user in db
// - generate qr code
// - return qr code with success message
pub async fn command_create_user(
    bot: Bot,
    msg: Message,
    bot_state: BotState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let is_admin = match msg.chat.username() {
        Some(username) => is_admin(username),
        None => false,
    };
    match is_admin {
        true => {
            let chat_id = msg.chat.id;
            let text = "Insert first name";
            bot.send_message(chat_id, text)
                .reply_parameters(ReplyParameters::new(msg.id))
                .await?;
        }
        false => {
            let chat_id = msg.chat.id;
            let text = "You are not admin";
            bot.send_message(chat_id, text).await?;
        }
    }
    Ok(())
}
