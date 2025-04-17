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

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = sc.next();
    let a: Vec<usize> = (0..n).map(|_| sc.next()).collect();
    let mut max = 0;
    let mut cur = 1;

    for i in 1..n {
        if a[i] >= a[i - 1] {
            cur += 1;
        } else {
            max = cur.max(max);
            cur = 1;
        }
    }

    max = max.max(cur);

    writeln!(out, "{}", max).unwrap()
}
