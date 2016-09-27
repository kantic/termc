extern crate termc_model;
extern crate termc_ui;

mod command_library;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use termc_model::get_result;
use termc_model::math_context::MathContext;
use termc_model::serialization::Serialization;
use termc_ui::{create_terminal_handle, TerminalUI};
use command_library::{CommandType, check_for_command};

#[cfg(unix)]
use termc_ui::unix::TerminalHandle;
#[cfg(windows)]
use termc_ui::windows::TerminalHandle;

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
    let mut context = MathContext::new();

    for (i, arg) in args.iter().enumerate() {

        match check_for_command(arg) {
            Some(c) => {
                match c {
                    CommandType::Exit => break,
                    CommandType::Load(path) => load_context::<TerminalHandle>(& path, & mut context, None),
                    CommandType::Save(path) => save_context::<TerminalHandle>(& path, & mut context, None)
                }
            },

            None => {
                match get_result(arg.trim(), & mut context) {
                    Ok(result) => {
                        match result {
                            Some(y) => results.push(y.to_string()),
                            None => ()
                        }
                    },
                    Err(err) => {
                        println!("In input {}\n: {}", i+1, err);
                        break;
                    }
                }
            }
        }
    }

    println!("{}", results.join(";"));
}

/// Starts termc in command line interactive mode.
fn start_interactive() {

    let mut terminal = create_terminal_handle();
    terminal.init();
    let mut context = MathContext::new();

    loop {
        let user_input = terminal.get_user_input();
        let user_input = user_input.trim();

        if user_input.len() == 0 {
            terminal.print_result("");
            continue;
        }

        match check_for_command(user_input) {
            Some(c) => {
                match c {
                    CommandType::Exit => break,
                    CommandType::Load(path) => {
                        load_context(&path, &mut context, Some(&mut terminal));
                        terminal.print_newline();
                    },
                    CommandType::Save(path) => {
                        save_context(& path, & mut context, Some(& mut terminal));
                        terminal.print_newline();
                    }
                }
            },

            None => {
                match get_result(& user_input, & mut context) {
                    Ok(result) => {
                        match result {
                            Some(y) => terminal.print_result(& format!("{}", y)),
                            None => terminal.print_newline()
                        }
                    },
                    Err(err) => {
                        terminal.print_error(err);
                    }
                }
            }
        }
    }

    terminal.end();
}

/// Loads the MathContext object from the specified file.
fn load_context<T: TerminalUI>(p: & str, context: & mut MathContext, terminal: Option<& mut T>) {
    let mut f = match File::open(p) {
        Ok(x) => x,
        Err(e) => {
            match terminal {
                Some(t) => t.print_error(e),
                None => println!("Error: {}", e)
            }
            return
        }
    };
    let mut s = String::new();
    match f.read_to_string(& mut s) {
        Ok(_) => (),
        Err(e) => {
            match terminal {
                Some(t) => t.print_error(e),
                None => println!("Error: {}", e)
            }
            return
        }
    }
    *context = match MathContext::deserialize(& s) {
        Ok(c) => c,
        Err(e) => {
            match terminal {
                Some(t) => t.print_error(e),
                None => println!("Error: {}", e)
            }
            MathContext::new()
        }
    }
}

/// Saves the MathContext object to the specified file.
fn save_context<T: TerminalUI>(p: & str, context: & mut MathContext, terminal: Option<& mut T>) {

    let serialization = match context.pretty_serialize() {
        Ok(s) => s,
        Err(e) => {
            match terminal {
                Some(t) => t.print_error(e),
                None => println!("Error: {}", e)
            }
            return
        }
    };

    let mut f = match File::create(p) {
        Ok(x) => x,
        Err(e) => {
            match terminal {
                Some(t) => t.print_error(e),
                None => println!("Error: {}", e)
            }
            return
        }
    };

    match f.write_all(serialization.as_ref()) {
        Ok(_) => (),
        Err(e) => {
            match terminal {
                Some(t) => t.print_error(e),
                None => println!("Error: {}", e)
            }
        }
    }
}