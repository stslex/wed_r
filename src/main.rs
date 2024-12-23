use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup};
use teloxide::utils::command::BotCommands;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot with Reply Keyboard...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_callback_query().endpoint(attend_invite_query))
        .branch(
            Update::filter_message()
                .branch(
                    dptree::entry()
                        .filter_command::<MenuCommand>()
                        .branch(dptree::case![MenuCommand::Start].endpoint(start))
                        .branch(dptree::case![MenuCommand::Help].endpoint(help))
                        .branch(dptree::case![MenuCommand::Invite].endpoint(invite)),
                )
                .branch(
                    dptree::entry()
                        .filter_command::<MenuCommonCommand>()
                        .branch(
                            dptree::case![MenuCommonCommand::BackToMainMenu]
                                .endpoint(main_menu_open),
                        ),
                ),
        );

    Dispatcher::builder(bot.clone(), handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn attend_invite_query(
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

async fn invite(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

async fn main_menu_open(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("main_menu_open");

    let all_commands = MenuCommand::bot_commands()
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

async fn help(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

async fn start(bot: Bot, msg: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    log::info!("start command");
    main_menu_open(bot, msg).await?;
    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum MenuCommand {
    #[command(description = "start the bot.")]
    Start,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start the bot.")]
    AllInfo,
    #[command(description = "start the bot.")]
    Location,
    #[command(description = "start the bot.")]
    Date,
    #[command(description = "invite status choose")]
    Invite,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum MenuCommonCommand {
    #[command(aliases = ["назад", "back_to_main_menu"], description = "вернуться в предыдущее меню.")]
    BackToMainMenu,
}
