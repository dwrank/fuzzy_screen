
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use std::io::{stdin, stdout};

use crate::screen::FuzzyScreen;

pub fn display(str_vec: &Vec<String>) {
    let mut fscreen = FuzzyScreen::new(stdout().into_raw_mode().unwrap(), str_vec);

    let stdin = stdin();
    fscreen.hide_cursor();
    fscreen.display();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => break,
            Key::Up => {
                fscreen.select_up();
            }
            Key::Down => {
                fscreen.select_down();
            }
            Key::PageUp => {
                fscreen.select_page_up();
            }
            Key::PageDown => {
                fscreen.select_page_down();
            }
            Key::Backspace => {
                fscreen.backspace_str();
            }
            Key::Char(ch) => {
                fscreen.append_str(ch);
            }
            _ => {}
        }
    }

    fscreen.show_cursor();
}
