use teloxide::types::Message;

pub mod admin;
pub mod callback_invite_query;
pub mod command_help;
pub mod command_invite;
pub mod command_open_main_menu;
pub mod model;
pub mod start;
mod tests;

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
