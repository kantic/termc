extern crate termc_model;
extern crate rustyline;
extern crate app_dirs;
extern crate colored;

use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use app_dirs::*;
use colored::*;
use rustyline::Editor;
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use termc_model::math_result::FormatIEEE754;

/// Defines the prompt.
static PROMPT : &'static str = ">>> ";

/// Defines the answer prefix
static ANS_PREFIX : &'static str = "ans = ";

/// Defines the maximum number of entries in the command history file.
static MAX_HISTORY_SIZE : usize = 250;

/// Information about the application.
static APP_INFO : AppInfo = AppInfo{name: "termc", author: "Jonas Kantic"};

/// Defines the formatting types for numbers.
#[derive(Clone)]
pub enum FormatType {
    /// Decimal representation.
    Dec,
    /// Octal representation.
    Oct,
    /// Hexadecimal representation.
    Hex,
    /// Binary representation.
    Bin,
    /// IEEE754 floating point binary representation.
    IEEE754,
    /// Scientific exponential representation.
    Exp,
    /// Undefined representation.
    Undefined
}

impl<'a> From<&'a str> for FormatType {
    fn from(s: &'a str) -> FormatType {
        if s == "bin" {
            FormatType::Bin
        }
        else if s == "oct" {
            FormatType::Oct
        }
        else if s == "hex" {
            FormatType::Hex
        }
        else if s == "ieee754" {
            FormatType::IEEE754
        }
        else if s == "exp" {
            FormatType::Exp
        }
        else if s == "dec" {
            FormatType::Dec
        }
        else {
            FormatType::Undefined
        }
    }
}

// The mode of the terminal ui.
#[derive(PartialEq)]
pub enum TerminalMode {
    /// In Interactive mode, readline will be used.
    Interactive,
    /// In Call mode, the results will be printed as a list (separated with ';').
    Call
}

#[macro_export]
macro_rules! format_result {
    ($typ:expr, $res:expr) => {{
        // typ: the format type
        // res: the result (MathResult)

        match $typ {
            FormatType::Dec | FormatType::Undefined => format!("{0}", $res),
            FormatType::Bin => format!("{0:#b}", $res),
            FormatType::Hex => format!("{0:#x}", $res),
            FormatType::Oct => format!("{0:#o}", $res),
            FormatType::Exp => format!("{0:E}", $res),
            FormatType::IEEE754 => format!("{0}", $res.ieee754_fmt()),
        }
    }};
    ($typ:expr, $res:ident, $ans_prefix:ident) => {{
        // typ: the format type
        // res: the result (MathResult)
        // ans_prefix: The prefix for the answer printing

        match $typ {
            FormatType::Dec | FormatType::Undefined => format!("{0}{1}", $ans_prefix, $res),
            FormatType::Bin => format!("{0}{1:#b}", $ans_prefix, $res),
            FormatType::Hex => format!("{0}{1:#x}", $ans_prefix, $res),
            FormatType::Oct => format!("{0}{1:#o}", $ans_prefix, $res),
            FormatType::Exp => format!("{0}{1:E}", $ans_prefix, $res),
            FormatType::IEEE754 => format!("{0}{1}", $ans_prefix, $res.ieee754_fmt())
        }
    }}
}

/// Prints the specified error.
fn print_error<T: Error>(err: T) {
        println!("{0}\n", err.to_string().red());
}

/// Prints the specified error message.
fn print_error_str(err: String) {
    println!("{0}\n", err.red());
}

/// Defines a handle for the terminal and provides functionalities for reading user input and writing results and error messages.
pub struct TerminalUI {
    mode: TerminalMode,
    editor: Option<Editor<FilenameCompleter>>,
    format_type: FormatType
}

impl TerminalUI {
    /// Creates a new TerminalUI instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_ui::{TerminalUI, TerminalMode};
    ///
    /// let tui = TerminalUI::new(TerminalMode::Interactive);
    /// ```
    pub fn new(mode: TerminalMode) -> Self {
        match mode {
            TerminalMode::Call => TerminalUI {mode: mode, editor: None, format_type: FormatType::Dec},

            TerminalMode::Interactive => {

                // NOTE: The following lines can not be used with rustyline 1.0.0
                //       All the config stuff is only available on the master branch of rustyline
                // configure and create read line editor
                /*let config = Config::builder()
                .history_ignore_space(true)
                .completion_type(CompletionType::List)
                .max_history_size(MAX_HISTORY_SIZE)
                .build();*/

                // create readline editor and configure history parameters
                let mut editor = Editor::new();
                editor = editor.history_ignore_dups(true)
                .history_ignore_space(true);
                editor.set_history_max_len(MAX_HISTORY_SIZE);
                
                // set the user input auto-completer
                let completer = FilenameCompleter::new();
                editor.set_completer(Some(completer));

                // load the history file if it exists and can be accessed
                // in case of a failure, no history will be loaded and an error message will be printed
                match get_history_file_path() {
                    Ok(pbuf) => {
                        let file_path = pbuf.as_path();
                        if file_path.exists() {
                            match editor.load_history(file_path) {
                                Ok(_) => (),
                                Err(e) => print_error_str(format!("Error: Could not load command history ({0}).", e))
                            }
                        }
                    },
                    Err(e) => print_error_str(format!("Error: Could not load command history ({0}).", e))
                }

                TerminalUI {mode: mode, editor: Some(editor), format_type: FormatType::Dec}
            }
        }
    }

    /// Retrieves the user input. This method should be used only in interactive mode, as otherwise the user will not be able to enter anything.
    /// Therefore, this method returns an empty String when it is called in call mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_ui::{TerminalUI, TerminalMode};
    ///
    /// let mut tui = TerminalUI::new(TerminalMode::Call);
    /// let user_input = tui.get_user_input();
    /// assert!(user_input == "");
    /// ```
    pub fn get_user_input(&mut self) -> String {

        match self.mode {
            // return an empty string in call mode
            TerminalMode::Call => String::from(""),

            // get the user input in ineractive mode by showing a prompt
            // save the user input in the history so that it can be saved in the history file when the program exits
            TerminalMode::Interactive => {
                let input = self.editor.as_mut().unwrap().readline(PROMPT);

                match input {
                    Ok(line) => {
                        self.editor.as_mut().unwrap().add_history_entry(line.as_ref());
                        line
                    },

                    // automatically call the exit command in case of CTRL-C or CTRL-D
                    Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                        String::from("exit")
                    },

                    Err(_) => {
                        String::from("")
                    }
                }
            }
        }
    }

    /// Prints the specified error in red color on the screen.
    /// NOTE: Coloring does not work in the CMD on Windows, but it works using PowerShell!
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_ui::{TerminalUI, TerminalMode};
    /// use std::io::{Error, ErrorKind};
    ///
    /// let mut tui = TerminalUI::new(TerminalMode::Call);
    /// let pseudo_error = Error::new(ErrorKind::PermissionDenied, "Oh dear!");
    /// tui.print_error(pseudo_error);
    /// ```
    pub fn print_error<T: Error>(&self, err: T) {
        print_error(err);
    }

    /// Prints the specified result. The result is prefixed with ANS_PREFIX.
    /// NOTE: This method should be used only in interactive mode because of the ANS_PREFIX. In call mode, use TerminalUI::print_results.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate termc_ui;
    /// extern crate termc_model;
    /// extern crate num;
    ///
    /// use termc_ui::{TerminalUI, TerminalMode};
    /// use termc_model::math_result::MathResult;
    /// use num::complex::Complex;
    ///
    /// fn main() {
    ///     let result = MathResult::from((4.1, 5.73));
    ///
    ///     let tui = TerminalUI::new(TerminalMode::Call);
    ///     tui.print_result(&result);
    /// }
    /// ```
    pub fn print_result<T: fmt::Display + fmt::Binary + fmt::LowerHex + fmt::UpperHex + fmt::Octal
                    + FormatIEEE754 + fmt::LowerExp + fmt::UpperExp>(&self, result: &T) {

        println!("{0}\n", &format_result!(self.format_type, result, ANS_PREFIX));
    }

    /// Prints the specified results seperated with ';'.
    /// NOTE: This method should be used only in call mode. In interactive mode, use TerminalUI::print_result.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate termc_ui;
    /// extern crate termc_model;
    /// extern crate num;
    ///
    /// use termc_ui::{TerminalUI, TerminalMode};
    /// use termc_model::math_result::MathResult;
    /// use num::complex::Complex;
    ///
    /// fn main() {
    ///     let results = vec![MathResult::from((4.1, 5.73)), MathResult::from((4.1, 0.0))];
    ///
    ///     let tui = TerminalUI::new(TerminalMode::Call);
    ///     tui.print_results(&results);
    ///     // Output will be: "4.1+5.73i;4.1"
    /// }
    /// ```
    pub fn print_results<T: fmt::Display + fmt::Binary + fmt::LowerHex + fmt::UpperHex + fmt::Octal
                     + FormatIEEE754 + fmt::LowerExp + fmt::UpperExp>(&self, results: &Vec<T>) {

        match self.mode {
            TerminalMode::Call => {
                let mut conc = String::from("");
                for r in results {
                    conc.push_str(&format_result!(self.format_type, r));
                    conc.push(';');
                }

                // pop the last ';'
                if conc.len() > 0 {
                    conc.pop();
                }

                println!("{0}", conc);
            },

            TerminalMode::Interactive => {
                // print each result seperately
                for r in results {
                    self.print_result(r);
                }
            }
        }
    }

    /// Prints the specified string.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_ui::{TerminalUI, TerminalMode};
    ///
    /// let tui = TerminalUI::new(TerminalMode::Call);
    /// tui.print("Hello World!\n");
    /// ```
    pub fn print(&self, s: &str) {
        print!("{0}", s);
    }

    /// Prints an acknowledge in green color.
    /// The intend of this method is to inform the user that a command has been executed successfully.
    /// Therefore, this method should be called after successful execution of a command.
    /// NOTE: Coloring does not work in the CMD on Windows, but it works using PowerShell!
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_ui::{TerminalUI, TerminalMode};
    ///
    /// let tui = TerminalUI::new(TerminalMode::Call);
    /// tui.print_cmd_ack();
    /// ```
    pub fn print_cmd_ack(&self) {
        println!("{0}\n", "Ok!".green());
    }

    /// Saves the user input history to the user config directory.
    /// NOTE: This method should only be used in interactive mode. In call mode, this method just returns Ok(()).
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_ui::{TerminalUI, TerminalMode};
    ///
    /// let mut tui = TerminalUI::new(TerminalMode::Call);
    /// tui.save_history_file().ok();
    /// ```
    pub fn save_history_file(&mut self) -> Result<(), AppDirsError> {

        if self.mode == TerminalMode::Interactive {
            let history_path_buf = get_history_file_path()?;
            let path = history_path_buf.as_path();
            self.editor.as_mut().unwrap().save_history(path).ok();
        }
        Ok(())
    }

    /// Sets the format type with which all further results are formatted.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate termc_ui;
    /// extern crate termc_model;
    /// extern crate num;
    ///
    /// use termc_ui::{TerminalUI, TerminalMode, FormatType};
    /// use termc_model::math_result::MathResult;
    /// use num::complex::Complex;
    ///
    /// fn main() {
    ///     let result = MathResult::from((10.0, 11.0));
    ///
    ///     let mut tui = TerminalUI::new(TerminalMode::Call);
    ///     tui.set_format_type(FormatType::Hex);
    ///     tui.print_result(&result);
    ///     // Output will be "0xA+0XBi"
    /// }
    /// ```
    pub fn set_format_type(&mut self, ft: FormatType) {
        self.format_type = ft;
    }
}

/// Gets the file path of the user input history file.
fn get_history_file_path() -> Result<PathBuf, AppDirsError> {

    let config_sub_dir = "termc";
    let mut path_buf = match get_app_dir(AppDataType::UserConfig, &APP_INFO, config_sub_dir) {
        Ok(p) => p,
        Err(_) => app_dir(AppDataType::UserConfig, &APP_INFO, config_sub_dir)?
    };

    path_buf.set_file_name("history");
    path_buf.set_extension("txt");

    Ok(path_buf)
}
