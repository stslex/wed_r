use teloxide::{
    dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, Handler},
    prelude::DependencyMap,
    types::Update,
};

use crate::{handlers::state::MenuCommonCommand, routes::command_open_main_menu};

use super::CommonCommandsHandler;

impl<'a: 'static> CommonCommandsHandler<'a>
    for Handler<
        'a,
        DependencyMap,
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
        DpHandlerDescription,
    >
{
    fn setup_common_commands(self) -> Self {
        self.branch(
            Update::filter_message().branch(
                dptree::entry()
                    .filter_command::<MenuCommonCommand>()
                    .branch(
                        dptree::case![MenuCommonCommand::BackToMainMenu]
                            .endpoint(command_open_main_menu::command),
                    ),
            ),
        )
    }
}
