extern crate termc_model;
extern crate termc_ui;

use std::env;
use termc_model::get_result;
use termc_model::math_context::MathContext;
use termc_ui::{create_terminal_handle, TerminalUI};

/// The main entry point.
pub fn main() {
    let args = get_arguments();

    // If there are command line arguments given, start in call mode.
    // Otherwise start in interactive mode.
    if args.len() > 0 {
        start_call(& args);
    }
    else {
        start_interactive();
    }
}

/// Returns the math expression command line arguments.
fn get_arguments() -> Vec<String> {

    let mut args_it = env::args();
    args_it.next(); // skip the first argument (path of the executable)
    args_it.collect()
}

/// Starts termc in command line call mode.
/// Prints a ';'-separated list with the results of the specified mathematical expressions.
fn start_call(args: & Vec<String>) {
    let mut results : Vec<String> = Vec::new();
    let context = MathContext::new();

    for (i, arg) in args.iter().enumerate() {
        match get_result(arg.trim(), & context) {
            Ok(result) => results.push(result.to_string()),
            Err(err) => {
                println!("In input {}\n: {}", i+1, err);
                break;
            }
        }
    }

    println!("{}", results.join(";"));
}

/// Starts termc in command line interactive mode.
fn start_interactive() {

    let mut terminal = create_terminal_handle();
    terminal.init();
    let context = MathContext::new();

    loop {
        let user_input = terminal.get_user_input();
        let user_input = user_input.trim();

        if user_input.len() == 0 {
            terminal.print_result("");
        }
        else if user_input.trim() == "exit" {
            break;
        }
        else {
            match get_result(& user_input, & context) {
                Ok(result) => {
                    terminal.print_result(& format!("{}", result));
                },
                Err(err) => {
                    terminal.print_error(err);
                }
            }
        }
    }

    terminal.end();
}