
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

use std::char;
use std::error::Error;
#[cfg(unix)]
use unix::TerminalHandle;
#[cfg(windows)]
use windows::TerminalHandle;

/// Defines the prompt.
static PROMPT : &'static str = ">>> ";
/// Defines the length of the prompt (number of characters)
static PROMPT_LEN : u16 = 4;

/// Defines the answer prefix
static ANS_PREFIX : &'static str = "ans: ";

/// Defines the ASCII number for the "Enter" key.
const ENTER_KEY : char = '\x0A';

/// Defines the interface with which the computation results and errors can be printed on the
/// terminal and with which the user input is fetched.
pub trait TerminalUI {
    /// Creates a new TerminalUI trait object instance.
    fn new() -> Self;
    /// Initializes the terminal
    fn init(& mut self);
    /// Finalize the terminal.
    fn end(& mut self);
    /// Gets the user input and manages the manipulation of the terminal.
    fn get_user_input(& mut self) -> String;
    /// Prints the specified result on the terminal.
    fn print_result(& mut self, result: &str);
    /// Prints the specified error on the terminal.
    fn print_error<T: Error>(& mut self, err: T);
    /// Prints the specified string.
    fn print_str(& mut self, s: &str);
}

/// Creates a new terminal handle that implements the TerminalUI trait.
pub fn create_terminal_handle() -> TerminalHandle {
    TerminalUI::new()
}