use teloxide::{
    dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, Handler},
    prelude::DependencyMap,
    types::Update,
};

use crate::{
    handlers::state::MenuAdminCommandState,
    routes::{
        admin::{
            create_user::command_create_user, get_all_created_users::command_get_all_created_users,
        },
        command_open_main_menu,
        start::command_start,
    },
};

use super::AdminMenuCommandsHandler;

impl<'a: 'static> AdminMenuCommandsHandler<'a>
    for Handler<
        'a,
        DependencyMap,
        Result<(), Box<dyn std::error::Error + Send + Sync>>,
        DpHandlerDescription,
    >
{
    fn setup_admin_menu_commands(self) -> Self {
        self.branch(
            Update::filter_message().branch(
                dptree::entry()
                    .filter_command::<MenuAdminCommandState>()
                    .branch(dptree::case![MenuAdminCommandState::Start].endpoint(command_start))
                    .branch(
                        dptree::case![MenuAdminCommandState::UserMenu]
                            .endpoint(command_open_main_menu::command),
                    )
                    .branch(
                        dptree::case![MenuAdminCommandState::CreatedUsers]
                            .endpoint(command_get_all_created_users),
                    )
                    .branch(
                        dptree::case![MenuAdminCommandState::CreateUser]
                            .endpoint(command_create_user),
                    ),
            ),
        )
    }
}
