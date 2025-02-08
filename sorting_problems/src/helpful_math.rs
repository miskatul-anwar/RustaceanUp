/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

#![allow(unused)]

use std::io::{stdin, BufRead};

fn rin_vec_int() -> Vec<usize> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    let mut vec: Vec<usize> = input
        .trim()
        .split('+')
        .filter_map(|i| i.parse().ok())
        .collect();
    vec.sort_unstable();
    vec
}

pub fn main() {
    let vec = rin_vec_int();
    let len = vec.len();

    for i in 0..len {
        if i != len - 1 {
            print!("{}+", vec[i])
        } else {
            println!("{}", vec[i])
        }
    }
}
