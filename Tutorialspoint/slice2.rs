use std::io::{stdin, BufRead};

fn main() {
    let mut input: String = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    let integers: Vec<i8> = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    let x: i8 = integers[0];
    let y: i8 = integers[1];
    println!("{}", x + y);
}
