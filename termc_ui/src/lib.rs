extern crate termc_model;
extern crate num;
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

static MAX_HISTORY_SIZE : usize = 250;

static APP_INFO : AppInfo = AppInfo{name: "termc", author: "Jonas Kantic"};

#[derive(Clone)]
pub enum FormatType {
    Dec,
    Oct,
    Hex,
    Bin,
    IEEE754,
    Exp,
    Unknown
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
            FormatType::Unknown
        }
    }
}

// The mode of the terminal ui
pub enum TerminalMode {
    // Interactive mode => readline will be used
    Interactive,
    // Call mode => results will be printed as a list (separated with ';')
    Call
}

#[macro_export]
macro_rules! format_result {
    ($typ:expr, $res:expr) => {{
        // typ: the format type
        // res: the result (MathResult)

        match $typ {
            FormatType::Dec | FormatType::Unknown => format!("{0}", $res),
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
            FormatType::Dec | FormatType::Unknown => format!("{0}{1}", $ans_prefix, $res),
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

pub struct TerminalUI {
    mode: TerminalMode,
    editor: Option<Editor<FilenameCompleter>>,
    format_type: FormatType
}

impl TerminalUI {
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

                let mut editor = Editor::new();
                editor = editor.history_ignore_dups(true)
                .history_ignore_space(true);
                editor.set_history_max_len(MAX_HISTORY_SIZE);
                
                // set the completer
                let completer = FilenameCompleter::new();
                editor.set_completer(Some(completer));

                // load the history file if the file exists and can be accessed
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

    pub fn get_user_input(&mut self) -> String {

        match self.mode {
            TerminalMode::Call => String::from(""),
            TerminalMode::Interactive => {
                let input = self.editor.as_mut().unwrap().readline(PROMPT);
                match input {
                    Ok(line) => {
                        self.editor.as_mut().unwrap().add_history_entry(line.as_ref());
                        line
                    },
                    Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                        // automatically call the exit command in case of CTRL-C or CTRL-D
                        String::from("exit")
                    },
                    Err(_) => {
                        String::from("")
                    }
                }
            }
        }
    }

    pub fn print_error<T: Error>(&self, err: T) {
        print_error(err);
    }

    pub fn print_result<T: fmt::Display + fmt::Binary + fmt::LowerHex + fmt::UpperHex + fmt::Octal
                    + FormatIEEE754 + fmt::LowerExp + fmt::UpperExp>(&self, result: Option<&T>) {

        match result {
            Some(r) => println!("{0}\n", &format_result!(self.format_type, r, ANS_PREFIX)),
            None => ()
        }
    }

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
                for r in results {
                    self.print_result(Some(r));
                }
            }
        }
    }

    pub fn print(&self, s: &str) {
        print!("{0}", s);
    }

    pub fn print_cmd_ack(&self) {
        println!("{0}\n", "Ok!".green());
    }

    pub fn save_history_file(&mut self) -> Result<(), AppDirsError> {

        let history_path_buf = try!(get_history_file_path());
        let path = history_path_buf.as_path();
        self.editor.as_mut().unwrap().save_history(path).ok();
        Ok(())
    }

    pub fn set_format_type(&mut self, ft: FormatType) {
        self.format_type = ft;
    }
}

fn get_history_file_path() -> Result<PathBuf, AppDirsError> {

    let config_sub_dir = "termc";
    let mut path_buf = match get_app_dir(AppDataType::UserConfig, &APP_INFO, config_sub_dir) {
        Ok(p) => p,
        Err(_) => try!(app_dir(AppDataType::UserConfig, &APP_INFO, config_sub_dir))
    };

    path_buf.set_file_name("history");
    path_buf.set_extension("txt");

    Ok(path_buf)
}