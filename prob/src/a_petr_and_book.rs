#![allow(unused)]
use std::{
    io::{stdin, stdout, BufWriter, Write},
    thread::panicking,
};

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

/*~~~~~~~~~~~~~*
 * CODE BELOW: *
 *~~~~~~~~~~~~~*/

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n: i32 = sc.next();
    let mut days = [0; 7];

    for i in 0..7 {
        days[i] = sc.next();
    }

    let mut pages = 0;
    let mut day = 0;

    loop {
        pages += days[day];
        day += 1;

        if pages >= n {
            break;
        }

        if day >= 7 {
            day %= 7
        }
    }

    writeln!(out, "{}", day).unwrap()
}
