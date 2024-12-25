use teloxide::{
    dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, Handler},
    prelude::DependencyMap,
    types::Update,
};

use crate::{commands::state::MenuCommonCommand, routes::main_menu_open};

use super::CommonCommands;

impl<'a: 'static> CommonCommands<'a>
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
                        dptree::case![MenuCommonCommand::BackToMainMenu].endpoint(main_menu_open),
                    ),
            ),
        )
    }
}
