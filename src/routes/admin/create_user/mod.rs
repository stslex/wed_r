use crate::config::BotState;
use crate::repository::admin::model::CreateUserRequestModel;
use crate::repository::admin::AdminRepository;
use crate::{config::CreateUserState, repository::admin::is_admin};
use teloxide::{prelude::Requester, types::Message, Bot};

mod tests;

use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

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
                //save firstname
                let user_create = CreateUserRequestModel {
                    username: "",
                    name: &firstname,
                };
                match bot_state.create_user(&user_create).await {
                    Ok(user) => {
                        log::info!("User created: {:?}", user);
                        bot.send_message(msg.chat.id, format!("User {} created", user))
                            .await?;
                        //generate qr code
                        bot.send_message(msg.chat.id, format!("Here is {} QrCode", firstname))
                            .await?;
                    }
                    Err(err) => {
                        log::error!("Error creating user: {:?}", err);
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
