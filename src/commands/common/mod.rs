mod handler;

pub trait CommonCommands<'a> {
    fn setup_common_commands(self) -> Self;
}
