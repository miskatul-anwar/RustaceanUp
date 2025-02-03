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

fn rin_vec_char() -> Vec<char> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    let vec: Vec<char> = input.trim().chars().collect();
    return vec;
}

fn to_string(vec: Vec<char>) -> String {
    return vec.iter().collect::<String>();
}

fn solve() {
    /* *************************(  Code Here  )************************* */
    let n = rin_int();
    let mut a = rin_vec_int();
    a.sort_unstable();

    if n % 2 != 0 {
        print!("{} ", a[n / 2]);
    }

    for i in (0..n / 2).rev() {
        print!("{} {} ", a[n - i - 1], a[i]);
    }

    println!();
}

pub fn main() {
    let t = rin_int();
    for _ in 0..t {
        solve();
    }
}
