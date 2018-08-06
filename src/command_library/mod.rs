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
    Format(FormatType),
    /// The Info command that lists all user defined constants and functions.
    Info
}

/// The CommandError enum.
#[derive(Debug)]
pub enum CommandError {
    /// Error that occurs when an unknown format is requested (e.g. the user types: "format abc")
    FormatError(String),
    /// Error that occurs when the loading of a serialized MathContext from a file or the deseialization process fails.
    LoadSerError(String),
    /// Error that occurs when the serialization of the MathContext or the writing of the target file fails.
    SaveSerError(String)
}

impl Error for CommandError {
    /// Returns the description of the error.
    fn description(& self) -> & str {
        match *self {
            CommandError::FormatError(_) => "Unknown number format.",
            CommandError::LoadSerError(_) => "Loading of serialization file failed.",
            CommandError::SaveSerError(_) => "Saving of serialization file failed."
        }
    }

    /// Returns the preceding error.
    fn cause(& self) -> Option<& Error> {
        match *self {
            CommandError::FormatError(_) => None,
            CommandError::LoadSerError(_) => None,
            CommandError::SaveSerError(_) => None
        }
    }
}

impl fmt::Display for CommandError {
    /// Implements the Display trait for CommandError.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        match self {
            &CommandError::FormatError(ref form) => {
                let c = (form.chars().count() as i32) - 1;
                let mut spaces = String::new();
                for _ in 0..c {
                    spaces.push(' ');
                }
                write!(f, "           {0}^~~~ Error: Unknown format \"{1}\"", spaces, form)
            },

            &CommandError::LoadSerError(ref err) | &CommandError::SaveSerError(ref err) => write!(f, "Error: {0}.", err)
        }
    }
}

/// Checks whether the specified input string represents a command.
pub fn check_for_command(s: & str, context: & mut MathContext, terminal: & mut TerminalUI, default_file: String) -> Result<Option<CommandType>, CommandError> {

    lazy_static!{
        static ref REGEX_EXIT : Regex = Regex::new("^exit$").unwrap();
        static ref REGEX_SAVE : Regex = Regex::new(r"^save(\s+(?P<path>.*))?$").unwrap();
        static ref REGEX_LOAD : Regex = Regex::new(r"^load(\s+(?P<path>.*))?$").unwrap();
        static ref REGEX_FORMAT : Regex = Regex::new(r"^format(\s+(?P<format>.*))?$").unwrap();
        static ref REGEX_INFO : Regex = Regex::new(r"^info$").unwrap();
    }

    if REGEX_EXIT.is_match(s) {
        Ok(Some(CommandType::Exit)) // signal exit
    }
    else if REGEX_INFO.is_match(s) {
        print_info(context, terminal); // print information about user defined symbols
        Ok(Some(CommandType::Info))
    }
    else if let Some(cap) = REGEX_LOAD.captures(s) {
        let path = match cap.name("path") {
            Some(g) => g.as_str().to_string(), // take user specified file
            None => default_file // take default file
        };
        load_context(&path, context)?;
        Ok(Some(CommandType::Load(path)))
    }
    else if let Some(cap) = REGEX_SAVE.captures(s) {
        let path = match cap.name("path") {
            Some(g) => g.as_str().to_string(), // take user specified file
            None => default_file // take default file
        };
        save_context(&path, context)?;
        Ok(Some(CommandType::Save(path)))
    }
    else if let Some(cap) = REGEX_FORMAT.captures(s) {
        let form = cap.name("format");
        if form.is_some() {
            // find out which format is specified in the command
            let ft = FormatType::from(form.unwrap().as_str());
            match ft {
                FormatType::Undefined => Err(CommandError::FormatError(form.unwrap().as_str().to_string())),
                _ => {
                    // set the specified format
                    switch_format(terminal, ft.clone());
                    Ok(Some(CommandType::Format(ft)))
                }
            }
        }
        else {
            Err(CommandError::FormatError(String::new()))
        }
    }
    else {
        Ok(None)
    }
}

/// Saves the MathContext object to the specified file.
fn save_context(p: & str, context: & mut MathContext) -> Result<(), CommandError> {

    let serialization = match serde_json::to_string_pretty(&context) {
        Ok(s) => s,
        Err(e) => return Err(CommandError::SaveSerError(format!("Unable to serialize the current conext ({0})", e)))
    };

    let mut f = match File::create(p) {
        Ok(x) => x,
        Err(e) => return Err(CommandError::SaveSerError(format!("Unable to save the serialized context ({0})", e)))
    };

    match f.write_all(serialization.as_ref()) {
        Ok(_) => Ok(()),
        Err(e) => Err(CommandError::SaveSerError(format!("Unable to write the serialized context to the specified file ({0})", e)))
    }
}

/// Loads the MathContext object from the specified file.
fn load_context(p: & str, context: & mut MathContext) -> Result<(), CommandError> {
    let mut f = match File::open(p) {
        Ok(x) => x,
        Err(e) => return Err(CommandError::LoadSerError(format!("Unable to open the specified file ({0})", e)))
    };
    let mut s = String::new();
    match f.read_to_string(& mut s) {
        Ok(_) => (),
        Err(e) => return Err(CommandError::LoadSerError(format!("Unable to read the specified file ({0})", e)))
    }

    let mut result : Result<(), CommandError> = Ok(());
    *context = match serde_json::from_str(&s) {
        Ok(c) => c,
        Err(e) => {
            result = Err(CommandError::LoadSerError(format!("Unable deserialize the specified serialization file ({0})", e)));
            MathContext::new()
        }
    };
    context.initialize();
    
    result
}

/// Switches the output print format of the numbers.
fn switch_format(terminal: & mut TerminalUI, t: FormatType) {
    terminal.set_format_type(t);
}

/// Prints all user defined constants and functions.
fn print_info(context: &MathContext, terminal: & TerminalUI) {

    let user_constants = context.get_user_constants();
    let mut constants_vec = Vec::new();
    for (ident, value) in user_constants {
        constants_vec.push(format!("{0} = {1}", ident, value));
    }

    let mut functions_vec = context.get_user_function_definitions();
    let mut all_definitions = constants_vec;
    all_definitions.append(&mut functions_vec);

    if all_definitions.len() > 0 {
        let all_definitions = all_definitions.join("\n");
        terminal.print(&format!("{0}\n", all_definitions));
    }
}
