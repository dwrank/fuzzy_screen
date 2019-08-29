pub mod controller;
pub mod screen;
pub mod items;
pub mod debug;

extern crate termion;
extern crate strsim;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
