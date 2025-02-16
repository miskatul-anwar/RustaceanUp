/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

#![allow(unused)]

use std::{
    collections::HashSet,
    io::{stdin, BufRead},
};

fn rin_int() -> usize {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
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
    /* *************************(  Code Here  )************************* */
    let (n, mut a, mut possible) = (rin_int(), rin_vec_int(), true);
    a.sort_unstable();

    for i in 0..n - 1 {
        if a[i + 1] - a[i] > 1 {
            possible = false
        }
    }

    if possible {
        println!("YES")
    } else {
        println!("NO")
    }
}

pub fn main() {
    let t = rin_int();
    for _ in 0..t {
        solve();
    }
}
