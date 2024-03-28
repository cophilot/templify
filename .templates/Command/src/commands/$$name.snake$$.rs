use crate::types::argument::Argument;
use crate::types::command::Command;
use crate::types::flag::Flag;
use crate::types::status::Status;

/// Definition of the $$name$$ command.
pub fn definition() -> Command {
    let mut $$name.snake$$_commmand = Command::new(
        vec!["$$name$$".to_string()],
        $$name.snake$$,
        "$$name$$-description".to_string(),
    );

    $$name.snake$$_commmand
}

pub fn $$name.snake$$(command: &Command) -> Status {
    let st = crate::utils::functions::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    Status::error("$$name$$ is not implemented yet.".to_string());
}
