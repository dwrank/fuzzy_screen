
extern crate fuzzy_screen;

use fuzzy_screen::controller;
use fuzzy_screen::items::ScreenItem;

fn main() {
    let mut items: Vec<ScreenItem> = Vec::new();
    let mut s: String;

    for i in 1..10 {
        if i < 3 {
            s = format!("alpha_{}", i);
        }
        else if i < 7 {
            s = format!("bravo_{}", i);
        }
        else {
            s = format!("charlie_{}", i);
        }

        items.push(ScreenItem::new(s));
    }

    controller::display(items);
}

