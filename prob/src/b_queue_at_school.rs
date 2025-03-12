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

    let (n, t) = (sc.next::<usize>(), sc.next::<i32>());
    let mut line: Vec<char> = sc.next::<String>().chars().collect();

    for _ in 0..t {
        let mut i = 1;

        while i < n {
            if line[i] == 'G' && line[i - 1] == 'B' {
                line[i] = 'B';
                line[i - 1] = 'G';
                i += 1
            }
            i += 1
        }
    }

    for ch in line {
        write!(out, "{}", ch).unwrap()
    }

    writeln!(out).unwrap()
}
