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
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve() {
    let n = rin_int();
    let mut count = 0;
    let mut trains = rin_vec_int();

    // Bubble Sort to count swaps
    for i in 0..n {
        for j in 0..n - i - 1 {
            if trains[j] > trains[j + 1] {
                trains.swap(j, j + 1);
                count += 1;
            }
        }
    }

    println!("Optimal train swapping takes {} swaps.", count);
}

pub fn main() {
    let mut t = rin_int();

    for _ in 1..=t {
        solve();
    }
}
