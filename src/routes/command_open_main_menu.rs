use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{KeyboardButton, KeyboardMarkup, Message},
    utils::command::BotCommands,
    Bot,
};

use crate::handlers::state::MenuCommandState;

pub async fn command(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("main_menu_open");

    let all_commands = MenuCommandState::bot_commands()
        .iter()
        .map(|c| KeyboardButton::new(c.command.clone()))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    let keyboard = KeyboardMarkup::new(all_commands).resize_keyboard().clone();

    bot.send_message(msg.chat.id, "Welcome! Choose an option:")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}
