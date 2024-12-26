mod handler;
mod tests;

pub trait MenuCommandsHandler<'a> {
    fn setup_main_commands(self) -> Self;
}
