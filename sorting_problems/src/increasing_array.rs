/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/
#![allow(unused)]

use std::{
    cmp::max,
    io::{stdin, BufRead},
};

fn rin_int() -> usize {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn rin_vec_int() -> Vec<i64> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let n = rin_int();
    let mut nums = rin_vec_int();

    let mut total = 0;
    let mut m = nums[0];

    for i in 1..n {
        total += max(0, m - nums[i]);
        m = max(m, nums[i]);
    }

    println!("{}", total);
}
