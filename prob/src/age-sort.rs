/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

#![allow(unused)]

use std::io::{stdin, BufRead};

fn rin_int() -> u64 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn rin_vec_int() -> Vec<i8> {
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
    let mut ages = rin_vec_int();
    ages.sort_unstable();

    for age in ages {
        print!("{age} ");
    }
    println!();
}

pub fn main() {
    let t = rin_int();
    for _ in 0..t {
        solve();
    }
}
