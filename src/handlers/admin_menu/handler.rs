use teloxide::{
    dispatching::{dialogue::InMemStorage, DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, Handler},
    prelude::{DependencyMap, Message},
    types::Update,
};

use crate::{
    config::AdminDialogue,
    handlers::state::MenuAdminCommandState,
    routes::{
        admin::{
            create_user::{command_create_user, handle_create_user_state},
            get_all_created_users::command_get_all_created_users,
            remove_user::{command_remove_user, handle_remove_user_state},
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
                    .branch(
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
                                dptree::case![MenuAdminCommandState::CreateUser].branch(
                                    dptree::entry()
                                        .enter_dialogue::<Message, InMemStorage<AdminDialogue>, AdminDialogue>()
                                        .endpoint(command_create_user),
                                )
                            )
                            .branch(
                                dptree::case![MenuAdminCommandState::RemoveUser].branch(
                                    dptree::entry()
                                        .enter_dialogue::<Message, InMemStorage<AdminDialogue>, AdminDialogue>()
                                        .endpoint(command_remove_user),
                                )
                            )
                    )
                    .branch(
                        dptree::entry()
                            .enter_dialogue::<Message, InMemStorage<AdminDialogue>, AdminDialogue>()
                            .branch(
                                dptree::case![AdminDialogue::CreateUser(state)]
                                .branch(dptree::endpoint(handle_create_user_state)),
                            )
                            .branch(
                                dptree::case![AdminDialogue::RemoveUser(state)]
                                .branch(dptree::endpoint(handle_remove_user_state)),
                            ),
                    ),
            ),
        )
    }
}
