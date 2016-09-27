
/// Defines the commands.
pub enum CommandType {
    /// The exit command.
    Exit,
    /// The load command (path).
    Load(String),
    /// The save command (path).
    Save(String)
}

/// Checks whether the specified input string represents a command.
pub fn check_for_command(s: & str) -> Option<CommandType> {
    if s.starts_with("exit") {
        Some(CommandType::Exit)
    }
    else if s.starts_with("load") || s.starts_with("save") {
        let mut copy = String::from(s);
        copy.drain(0..4);  // remove "load" or "save" from the beginning of the string
        let copy = String::from(copy.trim());

        if s.starts_with("load") {
            Some(CommandType::Load(copy))
        }
        else {
            Some(CommandType::Save(copy))
        }
    }
    else {
        None
    }
}