mod tests;

use log::error;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{KeyboardMarkup, Message, User},
    utils::command::BotCommands,
    Bot,
};

use crate::{
    config::BotState,
    handlers::state::MenuCommandState,
    repository::{model::UserRequestDataModel, UserRepository},
    utils::KeyboardButtonUtil,
};

pub async fn command_start(
    bot: Bot,
    msg: Message,
    bot_state: BotState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("start command");
    let user = match msg.from.clone() {
        Some(user) => user,
        None => {
            error!("Message from is None");
            bot.send_message(msg.chat.id, "Message from is None")
                .await?;
            return Ok(());
        }
    };
    process_start_user(bot, msg, user, bot_state).await
}

async fn process_start_user(
    bot: Bot,
    msg: Message,
    user: User,
    bot_state: BotState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let username = user.username.unwrap_or("".to_owned());
    let name = user.first_name;

    if name.is_empty() || username.is_empty() {
        error!("Name or username is empty");
        bot.send_message(msg.chat.id, "Name or username is empty")
            .await?;
        return Ok(());
    }

    let request_model = UserRequestDataModel {
        username: &username,
        name: &name,
    };

    let message = match bot_state.create_or_get_user(request_model).await {
        Ok(user) => format!("Welcome, {}! Choose an option:", user.name),
        Err(e) => {
            error!("Failed to create or get user: {}", e);
            "Welcome! Choose an option:".to_string()
        }
    };

    let all_commands = MenuCommandState::bot_commands().create_keyboard_buttons();
    let keyboard = KeyboardMarkup::new(all_commands).resize_keyboard().clone();

    bot.send_message(msg.chat.id, message)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
