/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

#![allow(unused)]

use std::{
    i32::MAX,
    io::{stdin, BufRead},
};

fn rin_int() -> usize {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn rin_vec_int() -> Vec<i32> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve() {
    /* *************************(  Code Here  )************************* */
    let mut rel = rin_vec_int();
    let n = rel[0];

    rel[0] = MAX;
    rel.sort_unstable();
    rel.pop();

    let median = rel[(n as usize) / 2];
    let mut ans = 0;

    for i in 0..n {
        ans += (median - rel[i as usize]).abs();
    }

    println!("{ans}");
}

pub fn main() {
    let t = rin_int();
    for _ in 0..t {
        solve();
    }
}
