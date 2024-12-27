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

    match msg.from {
        Some(user) => {
            let username = user.username.unwrap_or("".to_owned());
            let name = user.first_name;
            if name.is_empty() || username.is_empty() {
                error!("Name or username is empty");
                bot.send_message(msg.chat.id, "Name or username is empty")
                    .await?;
                return Ok(());
            }
            let message = match bot_state
                .create_or_get_user(UserRequestDataModel {
                    username: &username,
                    name: &name,
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
        }

        None => {
            error!("Message from is None");
            bot.send_message(msg.chat.id, "Message from is None")
                .await?;
            return Ok(());
        }
    }

    Ok(())
}
