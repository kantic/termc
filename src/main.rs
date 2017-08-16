#[macro_use]
extern crate lazy_static;
extern crate termc_model;
extern crate termc_ui;
extern crate serde_json;
extern crate regex;

mod command_library;

use std::env;
use std::path::Path;
use termc_model::get_result;
use termc_model::math_context::MathContext;
use termc_model::math_result::MathResult;
use termc_ui::{TerminalUI, TerminalMode};
use command_library::{CommandType, check_for_command};

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

fn build_default_ser_path(exe_path: &str) -> String {
    let default_fd = Path::new(exe_path).parent().unwrap(); // remove termc executable name
    let default_fn = Path::new("termc_context.json"); // define default file name
    default_fd.join(default_fn).to_str().unwrap().to_string() // join current path and default file name
}

/// Starts termc in command line call mode.
/// Prints a ';'-separated list with the results of the specified mathematical expressions.
fn start_call(args: & mut Vec<String>) {

    // compute default file-path for the serialization file
    let mut iter = args.iter();
    let path_str : String = iter.next().unwrap().to_string(); // get path of this executable
    let default_file = build_default_ser_path(&path_str);

    // create terminal handle
    let mut terminal = TerminalUI::new(TerminalMode::Call);

    let mut results : Vec<MathResult> = Vec::new();
    let mut context = MathContext::new();

    // for each argument given, evaluate it and store the results
    // if an error occurs for any of the given arguments, the evaluation of all arguments will be aborted
    for (i, arg) in iter.enumerate() {

        match check_for_command(arg, &mut context, &mut terminal, default_file.clone()) {
            Ok(k) => {
                match k {
                    Some(command_type) => {
                        match command_type {
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
                                terminal.print(&format!("In input {0}:\n", i+1));
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
fn start_interactive(path_str: String) {

    // compute default file-path for the serialization file
    let default_file = build_default_ser_path(&path_str);

    // create terminal handle
    let mut terminal = TerminalUI::new(TerminalMode::Interactive);
    // terminal.init();
    let mut context = MathContext::new();

    // REPL: take user input, evaluate it and print results / errors
    loop {
        let user_input = terminal.get_user_input();
        let user_input = user_input.trim();

        if user_input.len() == 0 {
            terminal.print_result::<MathResult>(None);
            continue;
        }

        match check_for_command(user_input, &mut context, &mut terminal, default_file.clone()) {
            Ok(k) => {
                match k {
                    Some(command_type) => {
                        match command_type {
                            CommandType::Exit => break,
                            _ => terminal.print_cmd_ack()
                        }
                    },

                    None => {
                        match get_result(& user_input, & mut context) {
                            Ok(result) => {
                                match result {
                                    Some(y) => terminal.print_result(Some(&y)),
                                    None => ()
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

    match terminal.save_history_file() {
        Ok(_) => (),
        Err(e) => terminal.print_error(e)
    }
}
