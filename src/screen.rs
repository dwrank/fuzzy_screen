
use termion::screen::{AlternateScreen};//, ToAlternateScreen, ToMainScreen};
use std::io::{Write};
use strsim::{hamming, levenshtein, normalized_levenshtein, osa_distance,
             damerau_levenshtein, normalized_damerau_levenshtein, jaro,
             jaro_winkler};

use crate::items::ScreenItems;
//use crate::debug::log;

pub struct FuzzyScreen<W: Write> {
    screen: AlternateScreen<W>,
    items: ScreenItems,
    cols: i32,  // display cols
    rows: i32,  // display rows
    search_str: String,
}

impl<W: Write> FuzzyScreen<W> {
    pub fn new(output: W, items: &Vec<String>) -> Self {
        let (cols, rows) = termion::terminal_size().unwrap();

        FuzzyScreen {
            screen: AlternateScreen::from(output),
            items: ScreenItems::new(items, rows as i32),
            cols: cols as i32,
            rows: rows as i32,
            search_str: String::new(),
        }
    }

    pub fn new_items(&mut self, items: &Vec<String>) {
        self.items = ScreenItems::new(items, self.rows);
        self.display();
    }
}

impl<W: Write> FuzzyScreen<W> {
    pub fn select_up(&mut self) {
        self.items.select_up();
        self.display();
    }

    pub fn select_down(&mut self) {
        self.items.select_down();
        self.display();
    }

    pub fn select_page_up(&mut self) {
        self.items.select_page_up();
        self.display();
    }

    pub fn select_page_down(&mut self) {
        self.items.select_page_down();
        self.display();
    }

    pub fn backspace_str(&mut self) {
        match self.search_str.pop() {
            Some(_) => {
                self.display();
            }
            None => {}
        }
    }

    pub fn append_str(&mut self, c: char) {
        self.search_str.push(c);
        self.display();
    }
}

impl<W: Write> FuzzyScreen<W> {
    pub fn hide_cursor(&mut self) {
        write!(self.screen, "{}", termion::cursor::Hide).unwrap();
    }

    pub fn show_cursor(&mut self) {
        write!(self.screen, "{}", termion::cursor::Show).unwrap();
    }

    pub fn display(&mut self) {
        let unselected_color = termion::color::Fg(termion::color::White);
        let selected_color = termion::color::Fg(termion::color::Green);

        // (col, row)
        let mut goto = (1, (self.items.max_display_items - self.items.num_display_items + 1) as u16);
        write!(self.screen, "{}", termion::clear::All).unwrap();
        //write!(self.screen, "{} {} {} {} {}\r\n", max_display_items, num_items, num_display_items, start, end).unwrap();

        //for (i, s) in str_vec.iter().enumerate() {
        let mut i = self.items.start();
        for s in self.items.str_vec_display() {
            //log(&format!("{} {}\n", i, s)[..]);
            if i == self.items.selected() {
                write!(self.screen, "{}",
                       selected_color).unwrap();
            }
            else {
                write!(self.screen, "{}",
                       unselected_color).unwrap();
            }

            write!(self.screen, "{goto}{index}: {file}{reset}\r\n",
                   goto = termion::cursor::Goto(goto.0, goto.1),
                   index = i + 1,
                   file = s,
                   reset = termion::color::Fg(termion::color::Reset)).unwrap();
            goto.1 += 1;
            i += 1;
        }

        write!(self.screen, "{}\r\n", self.items.num_items()).unwrap();

        write!(self.screen, "> {}", self.search_str).unwrap();

        self.screen.flush().unwrap();
    }
}
