use teloxide::{
    payloads::{AnswerCallbackQuerySetters, SendMessageSetters},
    prelude::{Request, Requester},
    types::{
        CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup,
        Message,
    },
    utils::command::BotCommands,
    Bot,
};

use crate::commands::state::{MenuCommandState, MenuCommonCommand};

pub async fn attend_invite_query(
    bot: Bot,
    q: CallbackQuery,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("attend_invite_query");
    match q.data {
        Some(data) => match data.as_str() {
            "attend_invite_yes" => {
                bot.answer_callback_query(q.id)
                    .text("You are going!")
                    .send()
                    .await?;
            }
            "attend_invite_no" => {
                bot.answer_callback_query(q.id)
                    .text("You are not going!")
                    .send()
                    .await?;
            }
            _ => {
                log::warn!("Unknown callback data: {}", data);
            }
        },
        None => {
            log::warn!("No callback data");
        }
    }
    Ok(())
}

pub async fn invite(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("invite");

    let keyboard = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("Да", "attend_invite_yes"),
        InlineKeyboardButton::callback("Нет", "attend_invite_no"),
    ]]);

    bot.send_message(msg.chat.id, "Would you go to the event?")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

pub async fn main_menu_open(
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

pub async fn help(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("help_menu_open");

    let all_commands = MenuCommonCommand::bot_commands()
        .iter()
        .map(|c| KeyboardButton::new(c.command.clone()))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    let keyboard = KeyboardMarkup::new(all_commands).resize_keyboard().clone();

    bot.send_message(msg.chat.id, "Help text goes here.")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn start(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("start command");
    main_menu_open(bot, msg).await?;
    Ok(())
}
