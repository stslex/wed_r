use teloxide::{
    payloads::AnswerCallbackQuerySetters,
    prelude::{Request, Requester},
    types::CallbackQuery,
    Bot,
};

enum CallbackQueryData {
    AttendInviteYes,
    AttendInviteNo,
    InsertFirstName,
}

pub async fn callback(
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
