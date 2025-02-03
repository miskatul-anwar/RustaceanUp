/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

use std::{
    collections::HashSet,
    io::{stdin, BufRead},
};

fn rin_i16() -> i16 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn rin_i8() -> i8 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
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

fn solve() -> &'static str {
    /* *************************(  Code Here  )************************* */
    let mut _n = rin_i8();

    let a = rin_vec_int();
    let b = rin_vec_int();

    let set_a: HashSet<i64> = a.into_iter().collect();
    let set_b: HashSet<i64> = b.into_iter().collect();

    let size_a = set_a.len();
    let size_b = set_b.len();

    if size_b == 1 {
        if size_a >= 3 {
            return "YES";
        } else {
            return "NO";
        }
    }

    if size_a == 1 {
        if size_b >= 3 {
            return "YES";
        } else {
            return "NO";
        }
    }

    "YES"
}

fn main() {
    let t = rin_i16();
    for _ in 0..t {
        println!("{}", solve());
    }
}
