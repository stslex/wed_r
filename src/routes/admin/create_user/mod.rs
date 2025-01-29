use std::fs;

use crate::config::BotState;
use crate::repository::admin::model::CreateUserRequestModel;
use crate::repository::admin::AdminRepository;
use crate::{config::CreateUserState, repository::admin::is_admin};
use image::{ImageBuffer, Luma};
use qrcodegen::QrCode;
use teloxide::types::InputFile;
use teloxide::{prelude::Requester, types::Message, Bot};

mod tests;

use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use uuid::Uuid;

pub async fn command_create_user(
    bot: Bot,
    msg: Message,
    dialogue: Dialogue<CreateUserState, InMemStorage<CreateUserState>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let is_admin = match msg.chat.username() {
        Some(username) => is_admin(username),
        None => false,
    };

    match is_admin {
        true => {
            let chat_id = msg.chat.id;
            let text = "Insert user name";

            bot.send_message(chat_id, text).await?;
            dialogue.update(CreateUserState::WaitingForUsername).await?;
        }
        false => {
            let chat_id = msg.chat.id;
            let text = "You are not admin";
            bot.send_message(chat_id, text).await?;
        }
    }
    Ok(())
}

pub async fn handle_create_user_state(
    bot: Bot,
    msg: Message,
    bot_state: BotState,
    dialogue: Dialogue<CreateUserState, InMemStorage<CreateUserState>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = dialogue.get_or_default().await;

    match state {
        Ok(state) => match state {
            CreateUserState::NotStarted => return Ok(()),
            CreateUserState::WaitingForUsername => wait_username(bot, msg, dialogue).await?,
            CreateUserState::WaitingForAccept { firstname } => {
                wait_for_accept(bot, msg, bot_state, dialogue, firstname).await?
            }
        },
        Err(err) => {
            log::error!("Error getting state: {:?}", err);

            bot.send_message(msg.chat.id, "something went wrong")
                .await?;
            dialogue.update(CreateUserState::NotStarted).await?;
        }
    }

    Ok(())
}

async fn wait_for_accept(
    bot: Bot,
    msg: Message,
    bot_state: BotState,
    dialogue: Dialogue<CreateUserState, InMemStorage<CreateUserState>>,
    firstname: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.text() {
        Some(accept_text) => {
            if accept_text.to_lowercase() == "no" || accept_text.to_lowercase() == "n" {
                bot.send_message(msg.chat.id, format!("User {} not created", firstname))
                    .await?;
            } else {
                let user_create = CreateUserRequestModel {
                    username: &firstname,
                    name: &firstname,
                };
                match bot_state.create_user(&user_create).await {
                    Ok(user) => {
                        log::info!("User created: {:?}", user);
                        bot.send_message(msg.chat.id, format!("User {} created", user))
                            .await?;
                        generate_qr_code(bot.clone(), user.uuid, msg.chat.id).await?;
                    }
                    Err(err) => {
                        log::error!("Error creating user: {:?}", err);
                        bot.send_message(
                            msg.chat.id,
                            format!("Error creating user {}: {}", firstname, err),
                        )
                        .await?;
                    }
                }
            }
            dialogue.update(CreateUserState::NotStarted).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please enter new user name")
                .await?;
            dialogue.update(CreateUserState::WaitingForUsername).await?;
        }
    }
    Ok(())
}

async fn wait_username(
    bot: Bot,
    msg: Message,
    dialogue: Dialogue<CreateUserState, InMemStorage<CreateUserState>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.text() {
        Some(name) => {
            bot.send_message(msg.chat.id, "Do you accept?").await?;
            dialogue
                .update(CreateUserState::WaitingForAccept {
                    firstname: name.to_string(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please enter your username")
                .await?;
        }
    }
    Ok(())
}

async fn generate_qr_code(
    bot: Bot,
    user_uuid: Uuid,
    chat_id: ChatId,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let bot_username = std::env::var("BOT_NAME").unwrap();
    let deep_link = format!("https://t.me/{bot_username}?start={user_uuid}");

    let qr = QrCode::encode_text(&deep_link, qrcodegen::QrCodeEcc::Medium).unwrap();
    let qr_size = qr.size() as u32;

    let scale = 10;
    let padding = 4 * scale;

    let img_size = (qr_size * scale) + 2 * padding;
    let mut image = ImageBuffer::from_pixel(img_size, img_size, Luma([255u8]));

    for y in 0u32..qr_size {
        for x in 0u32..qr_size {
            if qr.get_module(x as i32, y as i32) {
                for dy in 0..scale {
                    for dx in 0..scale {
                        image.put_pixel(
                            x * scale + padding + dx,
                            y * scale + padding + dy,
                            Luma([0u8]),
                        );
                    }
                }
            }
        }
    }

    let file_path = &format!("qr_code_{user_uuid}.png");
    image.save(file_path)?;

    let qr_file = InputFile::file(file_path);
    bot.send_photo(chat_id, qr_file)
        .caption(format!("QR code created:\n{}", deep_link))
        .await?;

    fs::remove_file(file_path)?;

    Ok(())
}
