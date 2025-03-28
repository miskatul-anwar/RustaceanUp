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

    let mut s: Vec<char> = sc.next::<String>().chars().into_iter().collect();
    let n = s.len();

    if n % 2 != 0 && s[n / 2] == '?' {
        s[n / 2] = 'x';
    }

    let (mut i, mut j) = (0, n - 1);

    while i < j {
        if s[i] == '?' && s[j] != '?' {
            s[i] = s[j];
        // } else if s[i] == '?' && s[j] == '?' {
        //     s[i] = 'x';
        //     s[j] = 'x';
        } else if s[i] != '?' && s[j] == '?' {
            s[j] = s[i];
        }
        i += 1;
        j -= 1;
    }

     let palindrome = (0..n / 2).all(|i| s[i] == s[n - 1 - i]);

    if palindrome {
        for i in s {
            write!(out, "{}", i).ok();
        }
        writeln!(out, "").ok();
    } else {
        writeln!(out, "-1").ok();
    }
}
