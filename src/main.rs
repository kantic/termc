#[macro_use]
extern crate lazy_static;
extern crate termc_model;
extern crate termc_ui;
extern crate serde_json;
extern crate regex;

mod command_library;

use std::env;
use termc_model::get_result;
use termc_model::math_context::MathContext;
use termc_model::math_result::MathResult;
use termc_ui::{create_terminal_handle, TerminalUI, TerminalMode};
use command_library::{CommandType, check_for_command};

#[cfg(unix)]
use termc_ui::unix::TerminalHandle;
#[cfg(windows)]
use termc_ui::windows::TerminalHandle;

/// The main entry point.
pub fn main() {
    let mut args = get_arguments();

    // If there are command line arguments given, start in call mode.
    // Otherwise start in interactive mode.
    if args.len() > 1 {
        start_call(& mut args);
    }
    else {
        let path = args.pop().unwrap(); // get path of this executable
        start_interactive(path);
    }
}

/// Returns the math expression command line arguments.
fn get_arguments() -> Vec<String> {

    let args_it = env::args();
    args_it.collect()
}

/// Starts termc in command line call mode.
/// Prints a ';'-separated list with the results of the specified mathematical expressions.
fn start_call(args: & mut Vec<String>) {

    let mut iter = args.iter();
    let path : String = iter.next().unwrap().to_string(); // get path of this executable
    let mut terminal = create_terminal_handle(TerminalMode::Call);
    terminal.init();

    let mut results : Vec<MathResult> = Vec::new();
    let mut context = MathContext::new();

    for (i, arg) in iter.enumerate() {

        match check_for_command::<TerminalHandle>(arg, &mut context, &mut terminal, & path) {
            Ok(k) => {
                match k {
                    Some(c) => {
                        match c {
                            CommandType::Exit => break,
                            _ => ()
                        }
                    },

                    None => {
                        match get_result(arg.trim(), & mut context) {
                            Ok(result) => {
                                match result {
                                    Some(y) => results.push(y),
                                    None => ()
                                }
                            },
                            Err(err) => {
                                terminal.print_str(&format!("In input {0}:", i+1));
                                terminal.print_newline();
                                terminal.print_error(err);
                                break;
                            }
                        }
                    }
                }
            },
            Err(e) => terminal.print_error(e)
        }
    }

   terminal.print_results(&results);
}

/// Starts termc in command line interactive mode.
fn start_interactive(path: String) {

    let mut terminal = create_terminal_handle(TerminalMode::Interactive);
    terminal.init();
    let mut context = MathContext::new();

    loop {
        let user_input = terminal.get_user_input();
        let user_input = user_input.trim();

        if user_input.len() == 0 {
            terminal.print_result::<MathResult>(None);
            continue;
        }

        match check_for_command(user_input, &mut context, &mut terminal, & path) {
            Ok(k) => {
                match k {
                    Some(c) => {
                        match c {
                            CommandType::Exit => break,
                            _ => terminal.print_newline()
                        }
                    },

                    None => {
                        match get_result(& user_input, & mut context) {
                            Ok(result) => {
                                match result {
                                    Some(y) => terminal.print_result(Some(&y)),
                                    None => terminal.print_newline()
                                }
                            },
                            Err(err) => {
                                terminal.print_error(err);
                            }
                        }
                    }
                }
            },
            Err(e) => terminal.print_error(e)
        }
    }

    terminal.end();
}
