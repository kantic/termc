extern crate termion;

use std::io::{Write, stdout, stdin};
use std::char;
use std::error::Error;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};
use termion::terminal_size;

/// Defines the prompt.
static PROMPT : &'static str = ">>> ";
/// Defines the length of the prompt (number of characters)
static PROMPT_LEN : u16 = 4;

/// Defines the answer prefix
static ANS_PREFIX : &'static str = "ans: ";

/// Defines the ASCII number for the "Enter" key.
const ENTER_KEY : char = '\x0A';

/// Defines the TerminalUI.
pub struct TerminalUI {
    /// User command history.
    inputs: Vec<String>,
    /// The x coordinate of the cursor position.
    x: u16,
    /// The y coordinate of the cursor position.
    y: u16,
    /// The y coordinate of the current prompt line.
    input_base_line: u16
}

// WARNING: termion cell indexing starts with 1
// => upper left corner has coordinates (1, 1) and NOT (0, 0)
impl<'a> TerminalUI {

    /// Creates a new TerminalUI instance.
    pub fn new() -> TerminalUI {
        TerminalUI {inputs: Vec::new(), x: 0, y: 0, input_base_line: 0}
    }

    /// Initializes the terminal (clears the terminal and positions the cursor).
    pub fn init(& mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        self.x = 1;
        self.y = 1;
        stdout.flush().unwrap();
    }

    /// Finalize the terminal.
    pub fn end(& mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        self.y += 1;
        self.x = 1;
        write!(stdout, "\n{}", termion::cursor::Goto(self.x, self.y)).unwrap();
        stdout.flush().unwrap();
    }

    /// Gets the user input and manages the manipulation of the terminal.
    pub fn get_user_input(& mut self) -> String {
        // get stdin, stdout and terminal size
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        let term_size = terminal_size().unwrap();

        // prepare current input, write prompt and update cursor coordinates
        let mut current_input : Vec<char> = Vec::new();
        write!(stdout, "{}{}", termion::cursor::Goto(1, self.y), PROMPT).unwrap();
        self.x = PROMPT_LEN + 1;
        self.input_base_line = self.y;
        stdout.flush().unwrap();
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
                    self.print_history_input(& output);
                    current_input = output.chars().collect();
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
                    self.print_history_input(& output);
                    current_input = output.chars().collect();
                },

                Key::Left => {
                    // move the cursor one step to the left
                    self.x -= 1;
                    if self.y > self.input_base_line {
                        if self.x < 1 {
                            // the cursor has to be set to the last position in the previous line
                            self.y -= 1;
                            self.x = term_size.0;
                        }
                    }
                    else {
                        // the cursor shall not move over the prompt
                        if self.x < PROMPT_LEN + 1 {
                            self.x = PROMPT_LEN + 1;
                        }
                    }
                    write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
                },

                Key::Right => {
                    // move the cursor one step to the right
                    self.x += 1;
                    let end_pos = self.get_cursor_end_pos(& current_input);
                    if self.y == end_pos.1 {
                        // the cursor shall not move more left than the last character of the input
                        if self.x > end_pos.0 {
                            self.x = end_pos.0;
                        }
                    }
                    else {
                        if self.y == self.input_base_line {
                            // the cursor shall not move more left than the last character of the
                            // input; here, the current input fits into the prompt line
                            if self.x > (current_input.len() as u16) + PROMPT_LEN + 1 {
                                self.x = (current_input.len() as u16) + PROMPT_LEN + 1;
                            }
                            // if the cursor is at the last position of the prompt line, move it to
                            // the first position of the next line
                            else if self.x > term_size.0 {
                                self.x = 1;
                                self.y += 1;
                            }
                        }
                        else {
                            // if the cursor is at the last position of the prompt line, move it to
                            // the first position of the next line
                            if self.x > term_size.0 {
                                self.x = 1;
                                self.y += 1;
                            }
                        }
                    }
                    write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
                },

                Key::Backspace => {

                    let end_pos = self.get_cursor_end_pos(& current_input);
                    // if the current input has multiple lines, clear them all
                    if end_pos.1 > self.input_base_line {
                        let mut y = end_pos.1;
                        while y != self.input_base_line {
                            write!(stdout, "{}{}", termion::cursor::Goto(1, y),
                                   termion::clear::CurrentLine).unwrap();
                            y -= 1;
                        }
                    }
                    if self.y > self.input_base_line {
                        // if the cursor is on the first position of a non-prompt line, then it
                        // has to be moved to the last position of the previous line
                        if self.x == 1 {
                            self.y -= 1;
                            self.x = term_size.0;
                        }
                        // just move the cursor one step to the left
                        else {
                            self.x -= 1;
                        }
                    }
                    // if the cursor is in the prompt line, we have to check that it does not move
                    // over the prompt
                    else {
                        if self.x > PROMPT_LEN + 1 {
                            self.x -= 1;
                        }
                        else {
                            continue;
                        }
                    }

                    // remove the character from the current input and update the terminal
                    let input_pos = self.get_input_cursor_pos();
                    current_input.remove(input_pos as usize);
                    let input_copy = current_input.clone();
                    let input: String = input_copy.into_iter().collect();
                    write!(stdout, "{}{}{}{}{}", termion::cursor::Goto(1, self.input_base_line),
                           termion::clear::CurrentLine, PROMPT, input, termion::cursor::Goto(self.x, self.y)).unwrap();
                },

                Key::Home => { // pos1
                    // move the cursor to the first character after the prompt
                    self.x = PROMPT_LEN + 1;
                    self.y = self.input_base_line;
                    write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
                },

                Key::End => {
                    // move the cursor after the last character of the current input
                    let end_pos = self.get_cursor_end_pos(& current_input);
                    self.x = end_pos.0;
                    self.y = end_pos.1;
                    write!(stdout, "{}", termion::cursor::Goto(self.x, self.y)).unwrap();
                },

                Key::Char(ENTER_KEY) => {
                    // write newline, update the cursor and break out of the loop
                    self.y += 1;
                    self.x = 1;
                    write!(stdout, "\n{}", termion::cursor::Goto(self.x, self.y)).unwrap();
                    stdout.flush().unwrap();
                    break;
                },

                Key::Char(c) => {
                    // insert the character into the current input and update the cursor position
                    let input_pos = self.get_input_cursor_pos();
                    current_input.insert(input_pos as usize, c);
                    self.x += 1;
                    if self.x > term_size.0 {
                        self.x = 1;
                        self.y += 1;
                    }
                    let input_copy = current_input.clone();
                    let input_str : String = input_copy.into_iter().collect();
                    write!(stdout, "{}{}{}{}{}", termion::clear::CurrentLine,
                           termion::cursor::Goto(1, self.input_base_line), PROMPT, input_str,
                           termion::cursor::Goto(self.x, self.y)).unwrap();
                },

                _ => {
                    continue;
                }
            }

            stdout.flush().unwrap();
        }

        let user_input : String = current_input.into_iter().collect();

        if user_input.len() > 0 && (self.inputs.len() == 0 || self.inputs[self.inputs.len() - 1 as usize] != user_input) {
            self.inputs.push(user_input.clone());
        }

        user_input
    }

    /// Prints the specified result on the terminal.
    pub fn print_result(& mut self, result: &str) {
        let lines = self.create_lines(result);
        let mut stdout = stdout().into_raw_mode().unwrap();
        self.x = 1;
        write!(stdout, "{}", ANS_PREFIX).unwrap();
        for line in lines {
            self.y += 1;
            write!(stdout, "{}\n{}", line, termion::cursor::Goto(self.x, self.y)).unwrap();
        }
        write!(stdout, "\n").unwrap();
        self.y += 1;
        stdout.flush().unwrap();

    }

    /// Prints the specified error on the terminal.
    pub fn print_error<T: Error>(& mut self, err: T) {
        let err_string = format!("{}", err);
        let lines = self.create_lines(& err_string);
        let mut stdout = stdout().into_raw_mode().unwrap();
        self.x = 1;
        for line in lines {
            self.y += 1;
            write!(stdout, "{}\n{}", line, termion::cursor::Goto(self.x, self.y)).unwrap();
        }
        write!(stdout, "\n").unwrap();
        self.y += 1;
        stdout.flush().unwrap();
    }

    /// Returns a vector of strings. These strings are sub-strings that
    /// match into one line of the terminal.
    fn create_lines(& self, input: & str) -> Vec<String> {
        let term_size = terminal_size().unwrap();
        let mut lines : Vec<String> = Vec::new();
        let mut s = String::new();
        for c in input.chars() {
            if c == '\n' || s.len() == (term_size.0 as usize) {
                lines.push(s);
                s = String::new();
            }
            else {
                s.push(c);
            }
        }
        if s.len() > 0 {
            lines.push(s);
        }
        lines
    }

    /// Prints the specified command on the terminal.
    fn print_history_input(& mut self, s: & String) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        self.x = 1;
        write!(stdout, "{}{}{}{}", termion::clear::CurrentLine,
               termion::cursor::Goto(self.x, self.y), PROMPT, s).unwrap();
        self.x = PROMPT_LEN + (s.len() as u16) + 1;
    }

    /// Returns the character position of the cursor in the current input.
    /// Assumes that the cursor is on a valid position of the current input.
    fn get_input_cursor_pos(& self) -> u16 {
        if self.y == self.input_base_line {
            self.x - 1 - PROMPT_LEN
        }
        else {
            let term_size = terminal_size().unwrap();
            (self.y - self.input_base_line) * term_size.0 + self.x - 1 - PROMPT_LEN
        }
    }

    /// Returns position after the last character of the current input.
    fn get_cursor_end_pos(& self, current_input: & Vec<char>) -> (u16, u16) {
        let len = PROMPT_LEN + (current_input.len() as u16);
        let term_size = terminal_size().unwrap();
        let lines : u16 = len / term_size.0;
        let offset : u16 = len % term_size.0;
        let y = self.input_base_line + lines;
        let x = offset + 1;
        (x, y)
    }
}