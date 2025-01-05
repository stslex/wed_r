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
    handlers::state::{MenuAdminCommandState, MenuCommandState},
    repository::{model::StartRequestModel, StartRepository},
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
    let msg_binding = msg.clone();
    let payload = get_payload(&msg_binding).await;
    let msg_binding = msg.clone();
    process_start_user(bot, &msg_binding, payload, user, bot_state).await
}

async fn get_payload<'a>(msg: &'a Message) -> &'a str {
    match msg.text() {
        Some(text) => {
            let payload = text.split_whitespace().collect::<Vec<&'a str>>();
            if payload.len() < 2 {
                return "";
            }
            return payload[1];
        }
        None => "",
    }
}

async fn process_start_user<'a>(
    bot: Bot,
    msg: &'a Message,
    payload: &'a str,
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

    let request_model = StartRequestModel {
        username: &username,
        name: &name,
        uuid: payload,
    };

    let result = bot_state.start(&request_model).await;

    let all_commands = match result.is_admin {
        true => MenuAdminCommandState::bot_commands(),
        false => MenuCommandState::bot_commands(),
    }
    .create_keyboard_buttons();

    let keyboard = KeyboardMarkup::new(all_commands).resize_keyboard().clone();

    bot.send_message(msg.chat.id, result.messege)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
