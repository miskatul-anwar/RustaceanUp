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

fn rin_vec_char() -> Vec<char> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    let vec: Vec<char> = input.trim().chars().collect();
    return vec;
}

pub fn main() {
    let (n, s) = (rin_int(), rin_vec_char());
    let (mut mx, mut pos) = (s[0], 0);

    for i in 1..n {
        if s[i] < mx {
            println!("YES");
            println!("{} {}", pos + 1, i + 1);
            return;
        } else {
            mx = s[i];
            pos = i;
        }
    }

    println!("NO");
}
