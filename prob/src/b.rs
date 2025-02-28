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

fn solve() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t: usize = sc.next();
    let mut results = Vec::new();

    for _ in 0..t {
        let n: usize = sc.next();
        let x: i32 = sc.next();
        let k: usize = sc.next();
        let commands: String = sc.next();

        let mut prefix = vec![0; n + 1];
        for i in 1..=n {
            prefix[i] = prefix[i - 1]
                + match commands.chars().nth(i - 1).unwrap() {
                    'L' => -1,
                    _ => 1,
                };
        }

        let mut first_reset = -1;
        for i in 1..=n {
            if x + prefix[i] == 0 {
                first_reset = i as i32;
                break;
            }
        }

        if first_reset == -1 || first_reset > k as i32 {
            results.push("0".to_string());
            continue;
        }

        let mut resets = 1;
        let mut time_used = first_reset as usize;

        let mut cycle_time = -1;
        for j in 1..=n {
            if prefix[j] == 0 {
                cycle_time = j as i32;
                break;
            }
        }

        if cycle_time == -1 {
            results.push(resets.to_string());
            continue;
        }

        let additional_cycles = (k - time_used) / cycle_time as usize;
        resets += additional_cycles;
        results.push(resets.to_string());
    }

    writeln!(out, "{}", results.join("\n")).unwrap();
}

fn main() {
    solve();
}
