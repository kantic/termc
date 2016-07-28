extern crate ncurses;

use ncurses::*;
use std::char;
use std::error::Error;


/// Defines the prompt.
static PROMPT : &'static str = ">>> ";
/// Defines the length of the prompt (number of characters)
static OFFSET : i32 = 4;

/// Defines the answer prefix
static ANS_PREFIX : &'static str = "ans: ";

/// Defines the ASCII number for the Enter-key (ncurses' KEY_ENTER is for the Enter-key at the
/// number block).
static ENTER_KEY : i32 = 10;

/// Defines the TerminalUI;
pub struct TerminalUI {
    /// User command history.
    inputs: Vec<String>
}

impl<'a> TerminalUI {

    /// Creates a new TerminalUI instance.
    pub fn new() -> TerminalUI {
        TerminalUI {inputs: Vec::new()}
    }

    /// Initializes the terminal with ncurses.
    pub fn init(&self) {
        initscr();
        noecho();
        //raw();
        scrollok(stdscr, true);
        keypad(stdscr, true);
    }

    /// Restores the terminal.
    pub fn end(&self) {
        endwin();
    }

    /// Gets the user input and manages the manipulation of the terminal.
    pub fn get_user_input(&mut self) -> String {

        let mut current_input : Vec<char> = Vec::new();
        printw(PROMPT);
        refresh();
        let mut cx = 0;
        let mut cy = 0;
        getyx(stdscr, &mut cy, &mut cx);
        let mut selected : u32 = 0;
        loop {
            let ch = getch();
            if ch == KEY_UP {  // go through the command history backwards
                if self.inputs.len() == 0 {
                    continue;
                }

                if selected == 0 {
                    selected = (self.inputs.len() as u32) - 1;
                }
                else {
                    selected -= 1;
                }

                // get and print the selected history command
                let output : &str = self.inputs[selected as usize].as_ref();
                wmove(stdscr, cy, 0);
                clrtoeol();
                mvprintw(cy, 0, format!("{}{}", PROMPT, output).as_ref());
                cx = getcurx(stdscr);
                current_input = String::from(output).chars().collect();
            }
            else if ch == KEY_DOWN {  // go through the command history forwards
                if self.inputs.len() == 0 {
                    continue;
                }

                if selected == (self.inputs.len() as u32) - 1 {
                    selected = 0;
                }
                else {
                    selected += 1;
                }

                // get and print the selected history command
                let output : &str = self.inputs[selected as usize].as_ref();
                wmove(stdscr, cy, 0);
                clrtoeol();
                mvprintw(cy, 0, format!("{}{}", PROMPT, output).as_ref());
                cx = getcurx(stdscr);
                current_input = String::from(output).chars().collect();
            }
            else if ch == KEY_LEFT {  // move cursor and stop at the prompt
                cx -= 1;
                if cx < OFFSET {
                    cx = OFFSET;
                }
                wmove(stdscr, cy, cx);
            }
            else if ch == KEY_RIGHT {  // move cursor and stop at the end of the user input
                cx += 1;
                if cx > (current_input.len() as i32) + OFFSET {
                    cx = (current_input.len() as i32) + OFFSET;
                }
                wmove(stdscr, cy, cx);
            }
            else if ch == KEY_BACKSPACE {  // remove a character
                if cx > OFFSET {
                    cx -= 1;
                    wmove(stdscr, cy, cx);
                    current_input.remove((cx - OFFSET) as usize);
                    delch();
                }
            }
            else if ch == ENTER_KEY {  // stop input loop
                let c = char::from_u32(ch as u32).expect("Invalid character!");
                wmove(stdscr, cy, OFFSET + (current_input.len() as i32));
                addch(c as u64);
                refresh();
                break;
            }
            else if keyname(ch).len() > 1 {  // ignore escaped characters
                continue;
            }
            else {  // take user input (a character the user types in)
                let c = char::from_u32(ch as u32).expect("Invalid character!");
                current_input.insert((cx - OFFSET) as usize, c);
                insch(c as u64);
                cx += 1;
                wmove(stdscr, cy, cx);
            }

            refresh();
        }

        let user_input : String = current_input.into_iter().collect();

        if user_input.len() > 0 && (self.inputs.len() == 0 || self.inputs[self.inputs.len() - 1 as usize] != user_input) {
            self.inputs.push(user_input.clone());
        }

        user_input
    }

    /// Prints the specified result on the terminal.
    pub fn print_result(&self, result: &str) {

        printw(format!("{}{}\n\n", ANS_PREFIX, result).as_ref());
        refresh();
    }

    /// Prints the specified error on the terminal.
    pub fn print_error<T: Error>(&self, err: T) {

        printw(format!("{}\n\n", err).as_ref());
        refresh();
    }
}