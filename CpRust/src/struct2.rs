use std::io::{stdin, BufRead};

struct Rectangle {
    h: u32,
    w: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.h * self.w
    }
    fn set_values(&mut self, h: u32, w: u32) {
        self.h = h;
        self.w = w;
    }
    /* More static methods goes here!*/
}

fn _rin() -> u32 {
    let mut input: String = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let mut rec1 = Rectangle { h: 0, w: 0 };
    println!("Enter the height & width: ");
    rec1.set_values(_rin(), _rin());
    println!("{}", rec1.get_area());
}
