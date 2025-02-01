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

pub fn main() {
    let mut n = rin_int();

    if n == 1 || n > 3 {
        for i in (2..=n).step_by(2) {
            print!("{i} ");
        }

        for i in (1..=n).step_by(2) {
            print!("{i} ");
        }

        println!()
    } else {
        println!("NO SOLUTION");
    }
}
