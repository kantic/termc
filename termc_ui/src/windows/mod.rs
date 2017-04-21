
use std::io;
use std::io::Write;
use std::error::Error;
use super::ANS_PREFIX;
use super::PROMPT;
use super::TerminalUI;
use super::TerminalMode;
use super::FormatType;

/// Defines the windows terminal handle.
pub struct TerminalHandle {
    format_type: FormatType
}

impl TerminalUI for TerminalHandle {

    /// Creates a new TerminalHandle instance.
    fn new(_: TerminalMode) -> TerminalHandle {
        TerminalHandle {format_type: FormatType::Dec}
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
    fn print_result<T: fmt::Display + fmt::Binary + fmt::LowerHex + fmt::UpperHex + fmt::Octal +
                        fmt::LowerExp + fmt::UpperExp>(& mut self, result: Option<&T>) {
        match result {
            Some(r) => println!("{}{}\n", ANS_PREFIX, r),
            None => println!("{}{}\n", ANS_PREFIX, "")
        }
    }

    /// Prints the specified results on the terminal, separated with ';'.
    fn print_results<T: fmt::Display + fmt::Binary + fmt::LowerHex + fmt::UpperHex + fmt::Octal +
                        fmt::LowerExp + fmt::UpperExp>(& mut self, results: &Vec<T>) {
        let mut conc = String::new();
        for r in results {
            conc += ";" + format_result!(self.format_type, r);
        }
        println!("{0}", conc);
    }

    /// Prints the specified error on the terminal.
    fn print_error<T: Error>(& mut self, err: T) {
        println!("{}\n", err);
    }

    /// Prints the specified string on the terminal.
    fn print_str(& mut self, s: & str) {
        println!("{}", s);
    }

    /// Prints newline on the terminal.
    fn print_newline(& mut self) {
        println!();
    }

    fn set_format_type(& mut self, t: FormatType) {
        self.format_type = t;
    }
}