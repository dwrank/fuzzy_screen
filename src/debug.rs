
use std::fs::OpenOptions;
use std::io::{Write};

pub fn log(msg: &str) {
    /*write!(self.screen, "{}", ToMainScreen).unwrap();
    write!(self.screen, "{}\r\n", msg).unwrap();
    write!(self.screen, "{}", ToAlternateScreen).unwrap();
    self.display();*/
    let mut f = OpenOptions::new().read(true).write(true).append(true).create(true).open("debug.log").unwrap();
    f.write(msg.as_bytes()).unwrap();
}
