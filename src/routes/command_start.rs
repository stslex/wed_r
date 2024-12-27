use log::error;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{KeyboardButton, KeyboardMarkup, Message},
    utils::command::BotCommands,
    Bot,
};

use crate::{
    config::BotState,
    handlers::state::MenuCommandState,
    repository::{model::UserRequestDataModel, UserRepository},
};

pub async fn command(
    bot: Bot,
    msg: Message,
    bot_state: BotState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("start command");

    let all_commands = MenuCommandState::bot_commands()
        .iter()
        .map(|c| KeyboardButton::new(c.command.clone()))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    let keyboard = KeyboardMarkup::new(all_commands).resize_keyboard().clone();

    let message = match bot_state
        .create_or_get_user(UserRequestDataModel {
            username: msg.chat.username().unwrap_or(""),
            name: msg.chat.first_name().unwrap_or(""),
        })
        .await
    {
        Ok(user) => format!("Welcome, {}! Choose an option:", user.name),
        Err(e) => {
            error!("Failed to create or get user: {}", e);
            "Welcome! Choose an option:".to_string()
        }
    };

    bot.send_message(msg.chat.id, message)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}
