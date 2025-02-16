#![allow(unused)]

use std::io::{stdin, BufRead};

fn rin_int() -> usize {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn rin_vec_int() -> Vec<u64> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve() {
    let n = rin_int();
    let mut a = rin_vec_int();

    a.sort_unstable();
    let (mut x, mut y) = (0, 0);

    for (i, &stick) in a.iter().enumerate() {
        if i % 2 == 0 {
            x += stick;
        } else {
            y += stick;
        }
    }

    let ans = x * x + y * y;
    println!("{}", ans);
}

pub fn main() {
    solve();
}
// FIXME: this code needs fix
