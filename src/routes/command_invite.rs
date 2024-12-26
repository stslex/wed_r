use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};

pub async fn command(
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
