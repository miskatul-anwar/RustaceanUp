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
    let (n, a) = (rin_int(), rin_vec_int());

    let min = *a.iter().min().unwrap();
    let mut amount = 0;

    a.iter().for_each(|i| amount += (i - min));
    println!("{}", amount);
}

pub fn main() {
    let t = rin_int();
    for _ in 0..t {
        solve();
    }
}
