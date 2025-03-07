#![allow(unused)]
use std::io::{stdin, stdout, BufWriter, Write};

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

    let n = sc.next();

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut count = 0;

    for i in 0..n {
        let t = sc.next();

        match t {
            1 => a += 1,
            2 => b += 1,
            3 => c += 1,
            _ => d += 1,
        }
    }

    count += d;

    if c < a {
        count += c;
        a -= c;
        c = 0;
    } else if a <= c {
        count += a;
        c -= a;
        a = 0;
    }

    if c > 0 {
        count += c;
        c = 0;
    }

    if b > 0 {
        count += b / 2;
        b %= 2;
    }

    let left = a + (b * 2);

    if left <= 4 && left > 0 {
        count += 1
    } else if left % 4 != 0 {
        count += (left / 4) + 1
    } else {
        count += left / 4
    }

    writeln!(out, "{}", count).unwrap()
}
