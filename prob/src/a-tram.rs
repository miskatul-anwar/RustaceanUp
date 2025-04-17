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

    let mut cap = 0;
    let mut max_cap = -1;

    for _ in 0..n {
        let a: i32 = sc.next();
        let b: i32 = sc.next();

        cap -= a;
        cap += b;

        max_cap = max_cap.max(cap)
    }

    writeln!(out, "{}", max_cap).unwrap()
}
