
use termion::screen::{AlternateScreen};//, ToAlternateScreen, ToMainScreen};
use std::io::{Write};
use strsim::{hamming, levenshtein, normalized_levenshtein, osa_distance,
             damerau_levenshtein, normalized_damerau_levenshtein, jaro,
             jaro_winkler};

use crate::items::{ScreenItem, ScreenManager};
//use crate::debug;
//use crate::debug::log;

pub struct FuzzyScreen<W: Write> {
    screen: AlternateScreen<W>,
    manager: ScreenManager,
    cols: i32,  // display cols
    rows: i32,  // display rows
    max_display_items: i32,
    search_str: String,
}

fn create_items(str_vec: Vec<String>) -> Vec<ScreenItem> {
    let mut items: Vec<ScreenItem> = Vec::new();
    for s in str_vec {
        items.push(ScreenItem::new(s));
    }

    items
}

impl<W: Write> FuzzyScreen<W> {
    pub fn new(output: W, items: Vec<ScreenItem>) -> Self {
        let (cols, rows) = termion::terminal_size().unwrap();
        let max_display_items = rows as i32 - 2;
        //debug::new();

        FuzzyScreen {
            screen: AlternateScreen::from(output),
            manager: ScreenManager::new(items, max_display_items),
            cols: cols as i32,
            rows: rows as i32,
            max_display_items: max_display_items,
            search_str: String::new(),
        }
    }
}

impl<W: Write> FuzzyScreen<W> {
    pub fn select_up(&mut self) {
        self.manager.select_up();
        self.display();
    }

    pub fn select_down(&mut self) {
        self.manager.select_down();
        self.display();
    }

    pub fn select_page_up(&mut self) {
        self.manager.select_page_up();
        self.display();
    }

    pub fn select_page_down(&mut self) {
        self.manager.select_page_down();
        self.display();
    }

    pub fn backspace_str(&mut self) {
        match self.search_str.pop() {
            Some(_) => {
                self.manager.fuzzy_sort(&self.search_str);
                self.display();
            }
            None => {}
        }
    }

    pub fn append_str(&mut self, c: char) {
        self.search_str.push(c);
        self.manager.fuzzy_sort(&self.search_str);
        self.display();
    }
}

//const UNSELECTED_FG_COLOR: termion::color::Fg<termion::color::White> = termion::color::Fg(termion::color::White);

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
        write!(self.screen, "{}", termion::clear::All).unwrap();
        let display_items: &[ScreenItem];
        match self.manager.display_items() {
            Some(items) => { display_items = items; }
            None => {
                self.display_end();
                return;
            }
        }

        let mut goto = (1, (self.max_display_items - display_items.len() as i32 + 1) as u16);

        let mut i = self.manager.start();
        let mut fill = String::new();
        for i in 0..self.cols {
            fill.push(' ');
        }

        for item in display_items {
            // goto line
            write!(self.screen, "{}",
                   goto = termion::cursor::Goto(goto.0, goto.1)).unwrap();

            if i == self.manager.selected() {
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

        self.display_end();
    }

    fn display_end(&mut self) {
        let unselected_fg_color = termion::color::Fg(termion::color::White);
        let selected_fg_color = termion::color::Fg(termion::color::White);
        let unselected_bg_color = termion::color::Bg(termion::color::Black);
        let selected_bg_color = termion::color::Bg(termion::color::LightBlack);
        let font_color = termion::color::Fg(termion::color::White);
        let selected_cursor_color = termion::color::Fg(termion::color::Green);
        let search_cursor_color = termion::color::Fg(termion::color::LightCyan);

        let mut goto = (1, (self.max_display_items + 1) as u16);

        write!(self.screen, "{goto}  {bg}{fg}{n1}/{n2}",
               goto = termion::cursor::Goto(goto.0, goto.1),
               bg = unselected_bg_color,
               fg = unselected_fg_color,
               n1 = self.manager.num_items(),
               n2 = self.manager.base_num_items()).unwrap();
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
}
