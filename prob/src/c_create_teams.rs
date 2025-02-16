/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

#![allow(unused)]

use std::io::{stdin, BufRead};

fn rin_int() -> usize {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn rin_vec_int() -> Vec<usize> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    return arr;
}

fn solve() {
    /* *************************(  Code Here  )************************* */
    let (n_x, mut a) = (rin_vec_int(), rin_vec_int());
    let (n, x) = (n_x[0], n_x[1]);

    a.sort_unstable_by(|a, b| b.cmp(a));
    let (mut cnt, mut team) = (1, 0);

    for i in 0..n {
        if cnt * a[i] >= x {
            team += 1;
            cnt = 0;
        }
        cnt += 1;
    }

    println!("{}", team);
}

pub fn main() {
    let t = rin_int();
    for _ in 0..t {
        solve();
    }
}
