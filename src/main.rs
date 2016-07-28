extern crate termc_model;
extern crate termc_ui;

use termc_model::get_result;
use termc_model::math_context::MathContext;
use termc_ui::TerminalUI;

pub fn main() {

    let mut terminal = TerminalUI::new();
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