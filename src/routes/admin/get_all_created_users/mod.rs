use crate::{config::BotState, repository::admin::AdminRepository};
use teloxide::{prelude::Requester, types::Message, Bot};

mod tests;

pub async fn command_get_all_created_users(
    bot: Bot,
    msg: Message,
    bot_state: BotState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("start command");
    let user = match msg.from.clone() {
        Some(user) => user,
        None => {
            bot.send_message(msg.chat.id, "Message from is None")
                .await?;
            return Ok(());
        }
    };

    let username = match user.username.clone() {
        Some(username) => username,
        None => {
            bot.send_message(msg.chat.id, "Username is None").await?;
            return Ok(());
        }
    };

    match bot_state.get_all_users(&username).await {
        Ok(users) => {
            let mut text_msg = "Index | username | name | is active | is accepted |\n".to_string();

            let text_users = users
                .iter()
                .enumerate()
                .map(|(index, user)| {
                    format!(
                        "{} | @{} | {} | {} | {} |",
                        index,
                        user.username,
                        user.name,
                        user.chat_id.is_some(),
                        user.is_accepted
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            text_msg.push_str("All users:\n");
            text_msg.push_str(&text_users);

            bot.send_message(msg.chat.id, text_msg).await?;
        }
        Err(e) => {
            log::error!("Cannot get all users: {}", e);
            bot.send_message(msg.chat.id, "Cannot get all users")
                .await?;
        }
    };
    Ok(())
}
