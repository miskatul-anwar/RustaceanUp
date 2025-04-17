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

type VecInt = Vec<i32>;
type VecBool = Vec<bool>;
type VecSize = Vec<usize>;
type Int = i32;
type Usize = usize;
type Int64 = i64;

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let t: Usize = sc.next();
    for _ in 0..t {
        let n: Usize = sc.next();
        let mut p: VecInt = vec![0; n + 1];
        for i in 1..=n {
            p[i] = sc.next();
        }
        let mut d: VecInt = vec![0; n + 1];
        for i in 1..=n {
            d[i] = sc.next();
        }
        let mut comp: VecSize = vec![0; n + 1];
        let mut cycle_sizes: VecSize = vec![];
        let mut cycle_id: Usize = 0;
        for i in 1..=n {
            if comp[i] != 0 {
                continue;
            }
            cycle_id += 1;
            let mut cur = i;
            let mut count: Usize = 0;
            while comp[cur] == 0 {
                comp[cur] = cycle_id;
                count += 1;
                cur = p[cur] as Usize;
            }
            cycle_sizes.push(count);
        }
        let mut forced: VecBool = vec![false; cycle_id + 1];
        let mut ans: Int64 = 0;
        for i in 1..=n {
            let idx: Usize = d[i] as Usize;
            let cid: Usize = comp[idx];
            if !forced[cid] {
                forced[cid] = true;
                ans += cycle_sizes[cid - 1] as Int64;
            }
            write!(out, "{} ", ans).unwrap();
        }
        writeln!(out).unwrap();
    }
}
