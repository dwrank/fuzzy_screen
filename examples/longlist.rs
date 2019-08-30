
extern crate fuzzy_screen;

use fuzzy_screen::controller;
use fuzzy_screen::items::ScreenItem;

fn main() {
    let mut items: Vec<ScreenItem> = Vec::new();
    let mut s: String;

    for i in 1..200 {
        if i < 20 {
            s = format!("alpha_{}", i);
        }
        else if i < 37 {
            s = format!("bravo_{}", i);
        }
        else if i < 87 {
            s = format!("charlie_{}", i);
        }
        else if i < 99 {
            s = format!("delta_{}", i);
        }
        else if i < 117 {
            s = format!("echo_{}", i);
        }
        else if i < 144 {
            s = format!("foxtrot_{}", i);
        }
        else if i < 163 {
            s = format!("golf_{}", i);
        }
        else if i < 190 {
            s = format!("hotel_{}", i);
        }
        else {
            s = format!("india_{}", i);
        }

        items.push(ScreenItem::new(s));
    }

    controller::display(items);
}

