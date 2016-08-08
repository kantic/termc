
extern crate termion;

use std::io::{Write, stdout, stdin};
use std::error::Error;
use super::ANS_PREFIX;
use super::PROMPT;
use super::PROMPT_LEN;
use super::ENTER_KEY;
use super::TerminalUI;
use self::termion::event::Key;
use self::termion::input::TermRead;
use self::termion::raw::{IntoRawMode};
use self::termion::terminal_size;

static WRT_ERR_MSG : &'static str = "Failed to write on terminal!";
static FLSH_ERR_MSG : &'static str = "Failed to flush terminal output buffer!";
static TERM_ERR_MSG : &'static str = "Could not gather information about the terminal!";
static STDOUT_ERR_MSG : &'static str = "Could not open stdout stream!";

/// Defines the unix terminal handle.
pub struct TerminalHandle {
    /// User command history.
    inputs: Vec<String>,
    /// The current user input.
    current_input: Vec<char>,
    /// The x coordinate of the cursor position.
    x: u16,
    /// The y coordinate of the cursor position.
    y: u16,
    /// The y coordinate of the current prompt line.
    input_base_line: u16
}

impl TerminalHandle {
    /// Returns a vector of strings. These strings are sub-strings that
    /// fit into one line of the terminal.
    fn create_lines(& self, input: & str) -> Vec<String> {
        let term_size = terminal_size().expect(TERM_ERR_MSG);
        let mut lines : Vec<String> = Vec::new();
        let mut s = String::new();
        let mut first_line = true;
        for c in input.chars() {
            if c == '\n' || (first_line && (s.len() as u16) + self.x - 1 == term_size.0) || (!first_line && (s.len() as u16) == term_size.0) {
                lines.push(s);
                s = String::new();
                first_line = false;
            }
            if c != '\n' {
                s.push(c);
            }
        }
        if s.len() > 0 {
            lines.push(s);
        }
        lines
    }

    /// Prints the specified command from the command history on the terminal.
    fn print_history_input(& mut self, s: & String, flush: bool) {
        self.clear_current_input(false);
        self.current_input.clear();
        for c in s.chars() {
            self.write_input_char(c, flush);
        }
    }

    /// Returns the character position of the cursor in the current input.
    /// Assumes that the cursor is on a valid position of the current input.
    fn get_input_cursor_pos(& self) -> u16 {
        if self.y == self.input_base_line {
            self.x - 1 - PROMPT_LEN
        }
        else {
            let term_size = terminal_size().expect(TERM_ERR_MSG);
            (self.y - self.input_base_line) * term_size.0 + self.x - 1 - PROMPT_LEN
        }
    }

    /// Returns position after the last character of the current input.
    fn get_cursor_end_pos(& self) -> (u16, u16) {
        let len = PROMPT_LEN + (self.current_input.len() as u16);
        let term_size = terminal_size().expect(TERM_ERR_MSG);
        let lines : u16 = len / term_size.0;
        let offset : u16 = len % term_size.0;
        let y = self.input_base_line + lines;
        let x = offset + 1;
        (x, y)
    }

    /// Applies terminal control operations (e.g., move the cursor) that are given in the specified
    /// string.
    fn control_terminal(& mut self, cntrl: & str, flush: bool) {
        let mut stdout = stdout().into_raw_mode().expect("");
        write!(stdout, "{}", cntrl).expect(WRT_ERR_MSG);

        if flush {
            stdout.flush().expect(FLSH_ERR_MSG);
        }
    }

    /// Moves the cursor one step to the left.
    fn move_cursor_left(& mut self, flush: bool) {
        let term_size = terminal_size().expect(TERM_ERR_MSG);
        if self.y == self.input_base_line {
            if self.x > PROMPT_LEN + 1 {
                self.x -= 1;
            }
            else {
                return;
            }
        }
        else {
            if self.x == 1 {
                self.y -= 1;
                self.x = term_size.0;
            }
            else {
                self.x -= 1;
            }
        }

        let (x, y) = (self.x, self.y);
        self.control_terminal(& format!("{}", termion::cursor::Goto(x, y)), flush);
    }

    /// Moves the cursor one step to the right.
    fn move_cursor_right(& mut self, flush: bool) {
        let term_size = terminal_size().expect(TERM_ERR_MSG);
        let end_pos = self.get_cursor_end_pos();
        if self.y < end_pos.1 {
            if self.x < term_size.0 {
                self.x += 1;
            }
            else {
                self.x = 1;
                self.y += 1;
            }
        }
        else {
            if self.x < end_pos.0 {
                self.x += 1;
            }
            else {
                return;
            }
        }

        let (x, y) = (self.x, self.y);
        self.control_terminal(& format!("{}", termion::cursor::Goto(x, y)), flush);
    }

    /// Clears the current user input from the terminal and leaves the input base line with only
    /// the prompt.
    fn clear_current_input(& mut self, flush: bool) {
        let len = PROMPT_LEN + (self.current_input.len() as u16);
        let term_size = terminal_size().expect(TERM_ERR_MSG);
        let lines : u16 = len / term_size.0 + 1;
        for i in 0..lines {
            let y = self.input_base_line + i;
            self.control_terminal(& format!("{}{}", termion::cursor::Goto(1, y), termion::clear::CurrentLine), false);
        }

        let input_base_line = self.input_base_line;
        self.x = 1;
        self.y = input_base_line;
        self.control_terminal(& format!("{}", termion::cursor::Goto(1, input_base_line)), false);
        self.write_prompt(flush);
    }

    /// Removes the character under the cursor from the current input.
    fn remove_input_char(& mut self, flush: bool) {
        if self.x == PROMPT_LEN + 1 && self.y == self.input_base_line {
            return;
        }

        let term_size = terminal_size().expect(TERM_ERR_MSG);
        let char_pos = self.get_input_cursor_pos() - 1;
        if self.y > self.input_base_line && self.x == 1 {
            self.y -= 1;
            self.x = term_size.0;
        }
        else {
            self.x -= 1;
        }

        let (x, y) = (self.x, self.y);
        self.current_input.remove(char_pos as usize);
        self.clear_current_input(false);
        let input_copy = self.current_input.clone();
        let input_str : String = input_copy.into_iter().collect();
        self.write_string(& input_str, false);
        self.x = x;
        self.y = y;
        self.control_terminal(& format!("{}", termion::cursor::Goto(x, y)), flush);
    }

    /// Adds the specified character at the current cursor position to the input.
    fn write_input_char(& mut self, c: char, flush: bool) {
        let term_size = terminal_size().expect(TERM_ERR_MSG);
        let char_pos = self.get_input_cursor_pos();
        if self.x == term_size.0 {
            self.y += 1;
            if self.y > term_size.1 {
                self.y = term_size.1;
            }
            self.x = 1;
        }
        else {
            self.x += 1;
        }

        let (x, y) = (self.x, self.y);
        self.current_input.insert((char_pos as usize), c);
        self.clear_current_input(false);
        let input_copy = self.current_input.clone();
        let input_str : String = input_copy.into_iter().collect();
        self.write_string(& input_str, false);
        self.x = x;
        self.y = y;
        self.control_terminal(& format!("{}", termion::cursor::Goto(x, y)), flush);
    }

    /// Writes the specified string on the terminal without adding it's content to the user input.
    fn write_string(& mut self, s: & str, flush: bool) {
        let mut lines = self.create_lines(s);
        if lines.len() == 0 {
            return;
        }
        let mut stdout = stdout().into_raw_mode().expect(STDOUT_ERR_MSG);
        let term_size = terminal_size().expect(TERM_ERR_MSG);
        let mut first_line = true;

        for line in lines.iter() {
            if !first_line {
                self.x = 1;
                self.y += 1;
                if self.y > term_size.1 {
                    self.y = term_size.1;
                    self.input_base_line -= 1;
                }
                write!(stdout, "\n{}", termion::cursor::Goto(1, self.y)).expect(WRT_ERR_MSG);
            }

            write!(stdout, "{}", line).expect(WRT_ERR_MSG);
            first_line = false;
        }

        let last_line = lines.pop().unwrap();
        self.x += last_line.len() as u16;
        if self.x > term_size.0 {
            self. x = self.x - term_size.0;
            self.y += 1;
            if self.y > term_size.1 {
                self.y = term_size.1;
                self.input_base_line -= 1;
                write!(stdout, "\n{}", termion::cursor::Goto(self.x, self.y)).expect(WRT_ERR_MSG);
            }
        }

        if flush {
            stdout.flush().expect(FLSH_ERR_MSG);
        }
    }

    /// Prints the prompt on the terminal.
    fn write_prompt(& mut self, flush: bool) {
        self.write_string(PROMPT, flush);
    }
}

impl TerminalUI for TerminalHandle {
    /// Creates a new TerminalHandle instance.
    fn new() -> TerminalHandle {
        TerminalHandle {inputs: Vec::new(), current_input: Vec::new(), x: 0, y: 0, input_base_line: 0}
    }

    /// Initializes the terminal (clears the terminal and positions the cursor).
    fn init(& mut self) {
        self.x = 1;
        self.y = 1;
        self.control_terminal(& format!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1)), true);
    }

    /// Finalize the terminal.
    fn end(& mut self) {
        let term_size = terminal_size().expect(TERM_ERR_MSG);
        self.y += 1;
        if self.y > term_size.1 {
            self.y = term_size.1;
            self.input_base_line -= 1;
        }
        let y = self.y;
        self.control_terminal(& format!("\n{}", termion::cursor::Goto(1, y)), true);
    }

    /// Gets the user input and manages the manipulation of the terminal.
    fn get_user_input(& mut self) -> String {
        // get stdin, stdout and terminal size
        let mut stdout = stdout().into_raw_mode().expect(STDOUT_ERR_MSG);
        let stdin = stdin();
        let term_size = terminal_size().expect(TERM_ERR_MSG);

        // prepare current input, write prompt and update cursor coordinates
        self.current_input = Vec::new();
        self.write_prompt(true);
        self.input_base_line = self.y;
        let mut selected : u32 = 0;

        // read the next character from stdin
        for ch in stdin.keys() {
            let c = match ch {
                Ok(x) => x,
                Err(_) => continue
            };

            match c {
                Key::Up => {
                    {
                        if self.inputs.len() == 0 {
                            continue;
                        }

                        if selected == 0 {
                            selected = (self.inputs.len() as u32) - 1;
                        }
                            else {
                            selected -= 1;
                        }
                    }

                    // get and print the selected history input
                    let output: String = String::from(self.inputs[selected as usize].as_ref());
                    self.print_history_input(& output, false);
                    self.current_input = output.chars().collect();
                },

                Key::Down => {
                    {
                        if self.inputs.len() == 0 {
                            continue;
                        }

                        if selected == (self.inputs.len() as u32) - 1 {
                            selected = 0;
                        }
                            else {
                            selected += 1;
                        }
                    }

                    // get and print the selected history input
                    let output: String = String::from(self.inputs[selected as usize].as_ref());
                    self.print_history_input(& output, false);
                    self.current_input = output.chars().collect();
                },

                Key::Left => {
                    // move the cursor one step to the left
                    self.move_cursor_left(false);
                },

                Key::Right => {
                    // move the cursor one step to the right
                    self.move_cursor_right(false);
                },

                Key::Backspace => {

                    self.remove_input_char(false);
                },

                Key::Home => { // pos1
                    // move the cursor to the first character after the prompt
                    self.x = PROMPT_LEN + 1;
                    self.y = self.input_base_line;
                    let (x, y) = (self.x, self.y);
                    self.control_terminal(& format!("{}", termion::cursor::Goto(x, y)), false);
                },

                Key::End => {
                    // move the cursor after the last character of the current input
                    let end_pos = self.get_cursor_end_pos();
                    self.x = end_pos.0;
                    self.y = end_pos.1;
                    let (x, y) = (self.x, self.y);
                    self.control_terminal(& format!("{}", termion::cursor::Goto(x, y)), false);
                },

                Key::Char(ENTER_KEY) => {
                    // write newline, update the cursor and break out of the loop
                    self.y += 1;
                    if self.y > term_size.1 {
                        self.y = term_size.1;
                        self.input_base_line -= 1;
                    }
                    self.x = 1;
                    let (x, y) = (self.x, self.y);
                    self.control_terminal(& format!("\n{}", termion::cursor::Goto(x, y)), true);
                    break;
                },

                Key::Char(c) => {
                    // insert the character into the current input and update the cursor position
                    self.write_input_char(c, false);
                },

                _ => {
                    continue;
                }
            }

            stdout.flush().expect(FLSH_ERR_MSG);
        }

        let input_copy = self.current_input.clone();
        let input_str : String = input_copy.into_iter().collect();

        if input_str.len() > 0 && (self.inputs.len() == 0 || self.inputs[self.inputs.len() - 1 as usize] != input_str) {
            self.inputs.push(input_str.clone());
        }

        input_str
    }

    /// Prints the specified result on the terminal.
    fn print_result(& mut self, result: &str) {
        self.write_string(& format!("{}{}", ANS_PREFIX, result), true);

        let term_size = terminal_size().expect(TERM_ERR_MSG);
        self.x = 1;
        self.y += 1;
        if self.y > term_size.1 {
            self.y = term_size.1;
            self.input_base_line -= 1;
        }
        self.y += 1;
        if self.y > term_size.1 {
            self.y = term_size.1;
            self.input_base_line -= 1;
        }
        let (x, y) = (self.x, self.y);
        self.control_terminal(& format!("\n\n{}", termion::cursor::Goto(x, y)), true);

    }

    /// Prints the specified error on the terminal.
    fn print_error<T: Error>(& mut self, err: T) {
        self.write_string(& format!("{}", err), true);

        let term_size = terminal_size().expect(TERM_ERR_MSG);
        self.x = 1;
        self.y += 1;
        if self.y > term_size.1 {
            self.y = term_size.1;
            self.input_base_line -= 1;
        }
        self.y += 1;
        if self.y > term_size.1 {
            self.y = term_size.1;
            self.input_base_line -= 1;
        }
        let (x, y) = (self.x, self.y);
        self.control_terminal(& format!("\n\n{}", termion::cursor::Goto(x, y)), true);
    }
}