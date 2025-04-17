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

fn compute_mex(a: &[i32], l: usize, r: usize) -> i32 {
    let mut present = [false; 4];
    for i in l..=r {
        if a[i] < 4 {
            present[a[i] as usize] = true;
        }
    }
    for x in 0..4 {
        if !present[x] {
            return x as i32;
        }
    }
    4
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let test_cases: i32 = sc.next();
    for _ in 0..test_cases {
        let n: usize = sc.next();
        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            a.push(sc.next::<i32>());
        }

        let mut operations = Vec::new();

        while a.len() > 3 {
            let has_zero = a.iter().any(|&x| x == 0);
            if has_zero {
                let pos = a.iter().position(|&x| x == 0).unwrap();
                if pos == 0 {
                    let m = compute_mex(&a, 0, 1);
                    operations.push((1, 2));
                    let mut new_a = Vec::with_capacity(a.len() - 1);
                    new_a.push(m);
                    new_a.extend_from_slice(&a[2..]);
                    a = new_a;
                } else {
                    let m = compute_mex(&a, pos - 1, pos);
                    operations.push((pos, pos + 1));
                    let mut new_a = Vec::with_capacity(a.len() - 1);
                    new_a.extend_from_slice(&a[..pos - 1]);
                    new_a.push(m);
                    new_a.extend_from_slice(&a[pos + 1..]);
                    a = new_a;
                }
            } else {
                let m = compute_mex(&a, 0, a.len() - 1);
                operations.push((1, a.len()));
                a = vec![m];
                break;
            }

            if a.len() == 3 {
                let p = compute_mex(&a, 0, 1);
                if p != 0 && a[2] != 0 {
                    operations.push((1, 2));
                    a = vec![p, a[2]];
                } else {
                    let q = compute_mex(&a, 1, 2);
                    if a[0] != 0 && q != 0 {
                        operations.push((2, 3));
                        a = vec![a[0], q];
                    } else {
                        operations.push((1, 3));
                        let m = compute_mex(&a, 0, 2);
                        a = vec![m];
                    }
                }
            }

            if a.len() == 2 {
                let m = compute_mex(&a, 0, 1);
                operations.push((1, 2));
                a = vec![m];
            }
        }

        writeln!(out, "{}", operations.len()).unwrap();
        for op in operations {
            writeln!(out, "{} {}", op.0, op.1).unwrap();
        }
    }
}
