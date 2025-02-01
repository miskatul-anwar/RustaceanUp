/*
This template is made by Miskatul Anwar<miskatul.anwar.csecu@gmail.com>
GitHub : https://github.com/miskatul-anwar

Thank You
*/

use std::io::{stdin, BufRead};

fn rin_int() -> i16 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn rin_vec_int() -> Vec<usize> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve() {
    /* ======================= Code Here ========================= */
    let n_x = rin_vec_int();
    let mut a = rin_vec_int();
    a.sort_unstable();

    let mut sum = 0;
    let mut _avg = 0;
    let mut ans = 0;

    for i in (0..n_x[0]).rev() {
        sum += a[i];
        _avg = sum / (n_x[0] - i);

        if _avg >= n_x[1] {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{ans}");
}

pub fn main() {
    let t = rin_int();

    for _ in 1..=t {
        solve();
    }
}
