
use termion::screen::{AlternateScreen};//, ToAlternateScreen, ToMainScreen};
use std::io::{Write};
use strsim::{hamming, levenshtein, normalized_levenshtein, osa_distance,
             damerau_levenshtein, normalized_damerau_levenshtein, jaro,
             jaro_winkler};

use crate::items::{ScreenItem, ScreenItems};
use crate::debug;
use crate::debug::log;

pub struct FuzzyScreen<W: Write> {
    screen: AlternateScreen<W>,
    items: ScreenItems,
    cols: i32,  // display cols
    rows: i32,  // display rows
    search_str: String,
    item_vec: Vec<ScreenItem>,
}

fn item_vec(str_vec: &Vec<String>) -> Vec<ScreenItem> {
    let mut item_vec: Vec<ScreenItem> = Vec::new();
    for s in str_vec {
        item_vec.push(ScreenItem::new(&s));
    }

    item_vec
}

impl<W: Write> FuzzyScreen<W> {
    pub fn new(output: W, str_vec: &Vec<String>) -> Self {
        let (cols, rows) = termion::terminal_size().unwrap();
        debug::new();

        let item_vec = item_vec(str_vec);

        FuzzyScreen {
            screen: AlternateScreen::from(output),
            items: ScreenItems::new(&item_vec, rows as i32),
            cols: cols as i32,
            rows: rows as i32,
            search_str: String::new(),
            item_vec: item_vec
        }
    }

    pub fn new_items(&mut self, str_vec: &Vec<String>) {
        self.items = ScreenItems::new(&item_vec(str_vec), self.rows);
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
                self.fuzzy_sort();
            }
            None => {}
        }
    }

    pub fn append_str(&mut self, c: char) {
        self.search_str.push(c);
        self.fuzzy_sort();
    }

    fn fuzzy_sort(&mut self) {
        for i in 0..self.item_vec.len() {
            self.item_vec[i].value = fuzzy_match(&self.search_str[..], &self.item_vec[i].name[..]);
            //self.item_vec[i].value = normalized_damerau_levenshtein(&self.search_str[..], &self.item_vec[i].name[..]);
        }
        self.item_vec.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());

        let mut n = 0;
        for (i, item) in self.item_vec.iter().enumerate() {
            if item.value != 0.0 {
                n = i;
                break;
            }
        }

        self.items = ScreenItems::new(&self.item_vec[n..], self.rows);
        self.display();
    }
}

// s1 is the search str
// s2 is the list item str
fn fuzzy_match(s1: &str, s2: &str) -> f64 {
    let mut value = 1.0;
    let l1 = s1.len();
    let l2 = s2.len();
    let mut penalty = 1.0 / l2 as f64;
    log(&format!("fuzzy_value: {} {}\n", s1 , s2));

    if l1 > l2 {
        value = 0.0;
    }
    else {
        let mut i2 = 0;
        let mut s2 = s2;
        for c1 in s1.chars() {
            s2 = &s2[i2..];
            log(&format!("i2: {} -> {}\n", i2 , s2));
            match s2.find(c1) {
                Some(n) => {
                    log(&format!("match n: {}\n", n));
                    value -= n as f64 * penalty;
                    log(&format!("new value: {}\n", value));
                    i2 = n + 1;
                    log(&format!("new i2: {}\n", i2));
                }
                None => {
                    value = 0.0;
                    break;
                }
            }
        }
    }

    value
}

impl<W: Write> FuzzyScreen<W> {
    pub fn hide_cursor(&mut self) {
        write!(self.screen, "{}", termion::cursor::Hide).unwrap();
    }

    pub fn show_cursor(&mut self) {
        write!(self.screen, "{}", termion::cursor::Show).unwrap();
    }

    pub fn display(&mut self) {
        let unselected_fg_color = termion::color::Fg(termion::color::White);
        let selected_fg_color = termion::color::Fg(termion::color::White);
        let unselected_bg_color = termion::color::Bg(termion::color::Black);
        let selected_bg_color = termion::color::Bg(termion::color::LightBlack);
        let font_color = termion::color::Fg(termion::color::White);
        let selected_cursor_color = termion::color::Fg(termion::color::Green);
        let search_cursor_color = termion::color::Fg(termion::color::LightCyan);

        // (col, row)
        let mut goto = (1, (self.items.max_display_items - self.items.num_display_items + 1) as u16);
        write!(self.screen, "{}", termion::clear::All).unwrap();

        let mut i = self.items.start();
        let mut fill = String::new();
        for i in 0..self.cols {
            fill.push(' ');
        }

        for item in self.items.item_vec_display() {
            // goto line
            write!(self.screen, "{}",
                   goto = termion::cursor::Goto(goto.0, goto.1)).unwrap();

            if i == self.items.selected() {
                write!(self.screen, "{style}{cursor_color}> {bg}{fg}",
                       style = termion::style::Bold,
                       cursor_color = selected_cursor_color,
                       bg = selected_bg_color,
                       fg = selected_fg_color).unwrap();
            }
            else {
                write!(self.screen, "  {bg}{fg}",
                       bg = unselected_bg_color,
                       fg = unselected_fg_color).unwrap();
            }

            let text = String::from(format!("{}: {} {}", i + 1, item.name, item.value));
            let fill_space = self.cols as usize - text.len() - 2;
            let line_fill = &fill[0..fill_space];

            write!(self.screen, "{text}{fill}{style_reset}",
                   text = text,
                   fill = line_fill,
                   style_reset = termion::style::Reset).unwrap();
            goto.1 += 1;
            i += 1;
        }

        write!(self.screen, "{goto}{bg}{fg}{text}",
               goto = termion::cursor::Goto(goto.0, goto.1),
               bg = unselected_bg_color,
               fg = unselected_fg_color,
               text = self.items.num_items()).unwrap();
        goto.1 += 1;

        write!(self.screen, "{goto}{search_cursor_color}>{fg_reset} {s}",
               goto = termion::cursor::Goto(goto.0, goto.1),
               s = self.search_str,
               search_cursor_color = search_cursor_color,
               fg_reset = termion::color::Fg(termion::color::Reset)).unwrap();
        goto.1 += 1;

        // reset colors
        write!(self.screen, "{goto}{bg}{fg}",
               goto = termion::cursor::Goto(goto.0, goto.1),
               bg = termion::color::Bg(termion::color::Reset),
               fg = termion::color::Fg(termion::color::Reset)).unwrap();

        self.screen.flush().unwrap();
    }

    fn write_text(&self, text: &str) {
    }
}
