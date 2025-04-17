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

    let t: usize = sc.next();

    for _ in 0..t {
        let _n: usize = sc.next();
        let s: String = sc.next();

        let mut num = 0;
        let mut dash = 0;
        let mut colon = 0;

        for c in s.chars() {
            if c == '_' {
                dash += 1;
            } else {
                colon += 1;
            }

            if dash >= 1 && colon >= 2 {
                dash -= 1;
                colon -= 2;
                num += 1;
            }
        }

        writeln!(out, "{}", num).unwrap();
    }
}
