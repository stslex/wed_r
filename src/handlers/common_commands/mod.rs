mod handler;

pub trait CommonCommandsHandler<'a> {
    fn setup_common_commands(self) -> Self;
}
