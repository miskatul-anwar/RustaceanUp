use std::io::stdin;

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut sc = Scanner::default();

    let x: usize = sc.next();
    let y: usize = sc.next();
    let mut b = 0;

    if x > y {
        b = (x * y) / x;
    } else {
        b = (x * y) / y;
    }

    if b % 2 == 0 {
        println!("Malvika");
    } else {
        println!("Akshat");
    }
}
