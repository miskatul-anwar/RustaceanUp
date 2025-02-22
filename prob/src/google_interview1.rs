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

    let n = sc.next::<usize>();
    let m = sc.next::<usize>();

    let mut land = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            land[i][j] = sc.next();
        }
    }

    let mut area = 0;

    for i in 0..n {
        for j in 0..m {
            if land[i][j] == 1 {
                let mut v_len = 0;
                let mut h_len = 0;

                for k in j..m {
                    match land[i][k] {
                        1 => v_len += 1,
                        _ => break,
                    }
                }

                for k in i..n {
                    match land[k][j] {
                        1 => h_len += 1,
                        _ => break,
                    }
                }

                area = area.max((v_len.min(h_len) as u32).pow(2))
            }
        }
    }

    writeln!(out, "{area}").unwrap()
}
