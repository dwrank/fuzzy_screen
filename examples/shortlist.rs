
extern crate fuzzy_screen;

use fuzzy_screen::controller;

fn main() {
    let mut str_vec: Vec<String> = Vec::new();
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
        str_vec.push(s);
    }

    controller::display(&str_vec);
}

