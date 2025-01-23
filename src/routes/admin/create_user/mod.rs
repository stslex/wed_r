use crate::config::BotState;
use teloxide::{types::Message, Bot};

mod tests;

pub fn command_create_user(
    bot: Bot,
    msg: Message,
    bot_state: BotState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // - route create user
    // - check if user is admin
    // - return callback for insert first name
    // - return callback for insert state / key (for group user)
    // - create user in db
    // - generate qr code
    // - return qr code with success message
    Ok(())
}
