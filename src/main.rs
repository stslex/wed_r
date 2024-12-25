use commands::CommandConfigure;
use routes::attend_invite_query;
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::prelude::*;

mod commands;
pub mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot with Reply Keyboard...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot.clone(),
        dptree::entry()
            .branch(Update::filter_callback_query().endpoint(attend_invite_query))
            .setup_commands(),
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
