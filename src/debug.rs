
use std::fs::{OpenOptions, File};
use std::io::{Write};

pub fn new() {
    OpenOptions::new().write(true).create(true).truncate(true).open("debug.log").unwrap();
}

/*
pub fn file() -> File {
    OpenOptions::new().read(true).write(true).append(true).create(true).open("debug.log").unwrap()
}

#[macro_export]
macro_rules! log {
    ($fmt:expr, $($name:ident),*) => { write!(&mut debug::file(), $fmt, $($name),*) }
}*/

pub fn log(msg: &String) {
    /*write!(self.screen, "{}", ToMainScreen).unwrap();
    write!(self.screen, "{}\r\n", msg).unwrap();
    write!(self.screen, "{}", ToAlternateScreen).unwrap();
    self.display();*/
    let mut f = OpenOptions::new().read(true).write(true).append(true).create(true).open("debug.log").unwrap();
    f.write(msg.as_bytes()).unwrap();
}
