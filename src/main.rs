use handlers::HandlerConfigure;

use teloxide::dispatching::Dispatcher;
use teloxide::prelude::*;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot with Reply Keyboard...");

    let bot = Bot::from_env();

    Dispatcher::builder(bot.clone(), dptree::entry().setup_handlers())
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
