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

fn play(x1: i32, x2: i32, x3: i32, x4: i32) -> i32 {
    let mut points = 0;

    if x1 > x2 {
        points += 1
    } else if x1 < x2 {
        points -= 1
    }

    if x3 > x4 {
        points += 1
    } else if x3 < x4 {
        points -= 1
    }

    if points > 0 {
        1
    } else {
        0
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();
    for _ in 1..=t {
        let (a1, a2, b1, b2) = (
            sc.next::<i32>(),
            sc.next::<i32>(),
            sc.next::<i32>(),
            sc.next::<i32>(),
        );

        let wins = play(a1, b1, a2, b2)
            + play(a1, b2, a2, b1)
            + play(a2, b1, a1, b2)
            + play(a2, b2, a1, b1);

        writeln!(out, "{}", wins).unwrap()
    }
}
