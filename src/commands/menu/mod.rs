mod handler;

pub trait MenuCommands<'a> {
    fn setup_main_commands(self) -> Self;
}
