use std::fs::File;
use std::io::{Read, Write};
use std::fmt;
use std::error::Error;
use serde_json;
use regex::Regex;
use termc_model::math_context::MathContext;
use termc_ui::FormatType;
use termc_ui::TerminalUI;


/// Defines the commands.
pub enum CommandType {
    /// The exit command.
    Exit,
    /// The load command (path).
    Load(String),
    /// The save command (path).
    Save(String),
    /// The format command (number format).
    Format(FormatType)
}

/// The CommandError enum.
#[derive(Debug)]
pub enum CommandError {
    /// Error that occurs when an unknown format is requested (e.g. the user types: "format abc")
    FormatError(String)
}

impl Error for CommandError {
    /// Returns the description of the error.
    fn description(& self) -> & str {
        match *self {
            CommandError::FormatError(_) => "Unknown number format.",
        }
    }

    /// Returns the preceding error.
    fn cause(& self) -> Option<& Error> {
        match *self {
            CommandError::FormatError(_) => None,
        }
    }
}

impl fmt::Display for CommandError {
    /// Implements the Display trait for CommandError.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        match self {
            &CommandError::FormatError(ref form) => {
                let c = form.chars().count() - 1;
                let mut spaces = String::new();
                for _ in 0..c {
                    spaces.push(' ');
                }
                write!(f, "           {0}^~~~ Error: Unknown format \"{1}\"", spaces, form)
            }
        }
    }
}

/// Checks whether the specified input string represents a command.
pub fn check_for_command<T: TerminalUI>(s: & str, context: & mut MathContext, terminal: & mut T) -> Result<Option<CommandType>, CommandError> {

    lazy_static!{
        static ref REGEX_EXIT : Regex = Regex::new("^exit$").unwrap();
        static ref REGEX_SAVE : Regex = Regex::new(r"^save(\s+(?P<path>.*))?$").unwrap();
        static ref REGEX_LOAD : Regex = Regex::new(r"^load(\s+(?P<path>.*))?$").unwrap();
        static ref REGEX_FORMAT : Regex = Regex::new(r"^format(\s+(?P<format>.*))?$").unwrap();
    }

    if REGEX_EXIT.is_match(s) {
        Ok(Some(CommandType::Exit))
    }
    else if let Some(cap) = REGEX_LOAD.captures(s) {
        let path = cap["path"].to_string();
        load_context(&path, context, terminal);
        Ok(Some(CommandType::Load(path)))
    }
    else if let Some(cap) = REGEX_SAVE.captures(s) {
        let path = cap["path"].to_string();
        save_context(&path, context, terminal);
        Ok(Some(CommandType::Save(path)))
    }
    else if let Some(cap) = REGEX_FORMAT.captures(s) {
        let ft = FormatType::from(&cap["format"]);
        match ft {
            FormatType::Unknown => Err(CommandError::FormatError(cap["format"].to_string())),
            _ => {
                switch_format(terminal, ft.clone());
                Ok(Some(CommandType::Format(ft)))
            }
        }
    }
    else {
        Ok(None)
    }
}

/// Saves the MathContext object to the specified file.
fn save_context<T: TerminalUI>(p: & str, context: & mut MathContext, terminal: & mut T) {

    let serialization = match serde_json::to_string_pretty(&context) {
        Ok(s) => s,
        Err(e) => {
            terminal.print_error(e);
            return
        }
    };

    let mut f = match File::create(p) {
        Ok(x) => x,
        Err(e) => {
            terminal.print_error(e);
            return
        }
    };

    match f.write_all(serialization.as_ref()) {
        Ok(_) => (),
        Err(e) => {
            terminal.print_error(e);
        }
    }
}

/// Loads the MathContext object from the specified file.
fn load_context<T: TerminalUI>(p: & str, context: & mut MathContext, terminal: & mut T) {
    let mut f = match File::open(p) {
        Ok(x) => x,
        Err(e) => {
            terminal.print_error(e);
            return
        }
    };
    let mut s = String::new();
    match f.read_to_string(& mut s) {
        Ok(_) => (),
        Err(e) => {
            terminal.print_error(e);
            return
        }
    }
    *context = match serde_json::from_str(&s) {
        Ok(c) => c,
        Err(e) => {
            terminal.print_error(e);
            MathContext::new()
        }
    };
    context.initialize();
}

/// Switches the output print format of the numbers.
fn switch_format<T: TerminalUI>(terminal: & mut T, t: FormatType) {
    terminal.set_format_type(t);
}