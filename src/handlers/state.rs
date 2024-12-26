use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum MenuCommandState {
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
pub enum MenuCommonCommand {
    #[command(aliases = ["назад", "back_to_main_menu"], description = "вернуться в предыдущее меню.")]
    BackToMainMenu,
}
