use std::error::Error;

use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use teloxide::Bot;

use crate::config::AdminDialogue;
use crate::config::BotState;
use crate::config::RemoveUserState;
use crate::repository::admin::is_admin;
use crate::repository::admin::model::UserResponseModel;
use crate::repository::admin::AdminRepository;

pub async fn command_remove_user(
    bot: Bot,
    bot_state: BotState,
    msg: Message,
    dialogue: Dialogue<AdminDialogue, InMemStorage<AdminDialogue>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    log::info!("Remove user command");

    let username = match msg.chat.username() {
        Some(username) => username,
        None => {
            bot.send_message(msg.chat.id, "Cannot get username").await?;
            dialogue.exit().await?;
            return Ok(());
        }
    };

    match is_admin(username) {
        true => {
            let users = match bot_state.get_all_users(username).await {
                Ok(users) => users,
                Err(e) => {
                    log::error!("Cannot get all users: {}", e);
                    return Ok(());
                }
            };

            let mut text_msg = "Index | username | name \n".to_string();

            let text_users = users
                .iter()
                .enumerate()
                .map(|(index, user)| format!("{} | @{} | {} |", index, user.username, user.name))
                .collect::<Vec<String>>()
                .join("\n");

            text_msg.push_str(&text_users);
            text_msg.push_str("\n\nInsert index of user to remove");

            bot.send_message(msg.chat.id, text_msg).await?;
            dialogue
                .update(AdminDialogue::RemoveUser(
                    RemoveUserState::WaitingForNumber { users },
                ))
                .await?;
        }
        false => {
            let chat_id = msg.chat.id;
            let text = "You are not admin";
            bot.send_message(chat_id, text).await?;
            dialogue.exit().await?;
        }
    }
    Ok(())
}

pub async fn handle_remove_user_state(
    bot: Bot,
    msg: Message,
    bot_state: BotState,
    dialogue: Dialogue<AdminDialogue, InMemStorage<AdminDialogue>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = dialogue.get_or_default().await;

    match state {
        Ok(state) => match state {
            AdminDialogue::RemoveUser(state) => match state {
                RemoveUserState::WaitingForNumber { users } => {
                    wait_number(bot, msg, dialogue, users).await?
                }
                RemoveUserState::WaitingForAccept { user } => {
                    wait_for_accept(bot, msg, bot_state, dialogue, user).await?
                }
            },
            _ => {
                bot.send_message(msg.chat.id, "something went wrong")
                    .await?;
                dialogue.exit().await?;
            }
        },
        Err(err) => {
            log::error!("Error getting state: {:?}", err);

            bot.send_message(msg.chat.id, "something went wrong")
                .await?;
            dialogue.exit().await?;
        }
    }

    Ok(())
}

async fn wait_number(
    bot: Bot,
    msg: Message,
    dialogue: Dialogue<AdminDialogue, InMemStorage<AdminDialogue>>,
    list_uuid: Vec<UserResponseModel>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let index = match msg.text() {
        Some(text) => match text.parse::<usize>() {
            Ok(index) => index,
            Err(_) => {
                bot.send_message(msg.chat.id, "Invalid index").await?;
                dialogue.exit().await?;
                return Ok(());
            }
        },
        None => {
            bot.send_message(msg.chat.id, "Cannot get message").await?;
            dialogue.exit().await?;
            return Ok(());
        }
    };
    let user = match list_uuid.get(index) {
        Some(user) => user,
        None => {
            bot.send_message(msg.chat.id, "Index out of bounds").await?;
            dialogue.exit().await?;
            return Ok(());
        }
    }
    .to_owned();

    bot.send_message(
        msg.chat.id,
        format!("Are you sure to remove user {}?", user.name),
    )
    .await?;

    dialogue
        .update(AdminDialogue::RemoveUser(
            RemoveUserState::WaitingForAccept { user },
        ))
        .await?;
    Ok(())
}

async fn wait_for_accept(
    bot: Bot,
    msg: Message,
    bot_state: BotState,
    dialogue: Dialogue<AdminDialogue, InMemStorage<AdminDialogue>>,
    user: UserResponseModel,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.text() {
        Some(accept_text) => {
            if accept_text.to_lowercase() == "no" || accept_text.to_lowercase() == "n" {
                bot.send_message(msg.chat.id, format!("User {} not removed", user.name))
                    .await?;
            } else {
                match bot_state.remove_user(&user.uuid).await {
                    Ok(()) => {
                        bot.send_message(msg.chat.id, format!("User {} removed", user.name))
                            .await?;
                    }
                    Err(err) => {
                        log::error!("Error creating user {} : {:?}", user.name, err);
                        bot.send_message(msg.chat.id, format!("Error removing user {}", err))
                            .await?;
                    }
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Please reinit remove user")
                .await?;
        }
    }
    dialogue.exit().await?;
    Ok(())
}
