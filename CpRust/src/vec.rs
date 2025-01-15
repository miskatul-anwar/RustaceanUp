use std::io::{stdin, BufRead};

fn _rin() -> Vec<i32> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

fn main() {
    for i in &v {
        println!("{i} ")
    }
}
