
use std::io;
use std::io::Write;
use std::error::Error;
use std::fmt;
use super::FormatIEEE754;
use super::ANS_PREFIX;
use super::PROMPT;
use super::TerminalUI;
use super::TerminalMode;
use super::FormatType;

/// Defines the windows terminal handle.
pub struct TerminalHandle {
    format_type: FormatType,
    mode: TerminalMode
}

impl TerminalUI for TerminalHandle {

    /// Creates a new TerminalHandle instance.
    fn new(mode: TerminalMode) -> TerminalHandle {
        TerminalHandle {mode: mode, format_type: FormatType::Dec}
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
        print!("{0}", PROMPT);
		io::stdout().flush().expect("Failed to flush terminal output buffer!");
        io::stdin().read_line(& mut user_input).expect("Failed to read user input!");
        user_input
    }

    /// Prints the specified result on the terminal.
    fn print_result<T: fmt::Display + fmt::Binary + fmt::LowerHex + fmt::UpperHex + fmt::Octal +
                        FormatIEEE754 + fmt::LowerExp + fmt::UpperExp>(& mut self, result: Option<&T>) {
        match result {
            Some(r) => {
                match self.mode {
                    TerminalMode::Call => {
                        self.print_str(&format_result!(self.format_type, r));
                        self.print_newline();
                    },
                    TerminalMode::Interactive => {
                        self.print_str(&format_result!(self.format_type, r, ANS_PREFIX));
                        self.print_newline();
                    }
                }
            },
            None => {
                match self.mode {
                    _ => self.print_newline()
                }
            }
        }
    }

    /// Prints the specified results on the terminal, separated with ';'.
    fn print_results<T: fmt::Display + fmt::Binary + fmt::LowerHex + fmt::UpperHex + fmt::Octal +
                        FormatIEEE754 + fmt::LowerExp + fmt::UpperExp>(& mut self, results: &Vec<T>) {
        match self.mode {
            TerminalMode::Call => {
                let mut conc = String::new();
                for r in results {
                    conc.push_str(&format_result!(self.format_type, r));
                    conc.push(';');
                }
                conc.pop();  // remove the last ';'
                self.print_str(&conc);
                self.print_newline();
            },
            TerminalMode::Interactive => {
                for r in results {
                    self.print_result(Some(r));
                }
            }
        }
    }

    /// Prints the specified error on the terminal.
    fn print_error<T: Error>(& mut self, err: T) {
        println!("{0}\n", err);
    }

    /// Prints the specified string on the terminal.
    fn print_str(& mut self, s: & str) {
        println!("{0}", s);
    }

    /// Prints newline on the terminal.
    fn print_newline(& mut self) {
        println!();
    }

    fn set_format_type(& mut self, t: FormatType) {
        self.format_type = t;
    }
}