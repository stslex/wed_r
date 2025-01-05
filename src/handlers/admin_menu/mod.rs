mod handler;

pub trait AdminMenuCommandsHandler<'a> {
    fn setup_admin_menu_commands(self) -> Self;
}
