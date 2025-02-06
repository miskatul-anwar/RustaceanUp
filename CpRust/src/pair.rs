use std::io::{stdin, BufRead};

struct Pair {
    value1: i32,
    value2: i32,
}

impl Pair {
    fn set_value(&mut self, value1: i32, value2: i32) {
        self.value1 = value1;
        self.value2 = value2;
    }
    fn show(&self) {
        println!("<{}, {}>", self.value1, self.value2);
    }
}

fn _rin() -> i32 {
    let mut input = String::new();
    stdin()
        .lock()
        .read_line(&mut input)
        .expect("Please enter values!");
    input.trim().parse().expect("Input values aren't integers")
}

fn main() {
    println!("Enter the values 1 & 2: ");

    let mut p1 = Pair {
        value1: 0,
        value2: 0,
    };

    let value1 = _rin();
    let value2 = _rin();

    p1.set_value(value1, value2);
    p1.show();
}

