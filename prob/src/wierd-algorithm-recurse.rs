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

fn algo(n: i64, steps: &mut Vec<i64>) {
    if n == 1 {
        return;
    } else if n % 2 == 0 {
        steps.push(n / 2);
        algo(n / 2, steps);
    } else {
        steps.push(n * 3 + 1);
        algo(n * 3 + 1, steps);
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = sc.next::<i64>();
    let mut steps: Vec<i64> = Vec::new();
    steps.push(n);

    algo(n, &mut steps);

    for i in steps {
        write!(out, "{i} ").unwrap()
    }

    writeln!(out, "").unwrap()
}
