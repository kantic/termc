
use std::io;
use std::io::Write;
use std::error::Error;
use super::ANS_PREFIX;
use super::PROMPT;
use super::TerminalUI;

/// Defines the windows terminal handle.
pub struct TerminalHandle {}

impl TerminalUI for TerminalHandle {

    /// Creates a new TerminalHandle instance.
    fn new() -> TerminalHandle {
        TerminalHandle {}
    }

    /// Initializes the terminal.
    fn init(& mut self) {
    }

    /// Finalize the terminal.
    fn end(& mut self) {
        println!("\n");
    }

    /// Gets the user input.
    fn get_user_input(& mut self) -> String {
        let mut user_input = String::new();
        print!("{}", PROMPT);
		io::stdout().flush().expect("Failed to flush terminal output buffer!");
        io::stdin().read_line(& mut user_input).expect("Failed to read user input!");
        user_input
    }

    /// Prints the specified result on the terminal.
    fn print_result(& mut self, result: &str) {
        println!("{}{}\n", ANS_PREFIX, result);
    }

    /// Prints the specified error on the terminal.
    fn print_error<T: Error>(& mut self, err: T) {
        println!("{}\n", err);
    }

    /// Prints the specified string on the terminal.
    fn print_str(& mut self, s: & str) {
        println!("{}", s);
    }
}