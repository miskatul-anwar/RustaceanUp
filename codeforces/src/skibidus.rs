/*~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 *~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*/

#![allow(unused)]

use std::io::{stdin, BufRead};

macro_rules! rin {
    ($t:ty) => {{
        let mut input = String::new();
        stdin().lock().read_line(&mut input).unwrap();
        input.trim().parse::<$t>().unwrap()
    }};
}

macro_rules! rin_vec {
    ($t:ty) => {{
        let mut input = String::new();
        stdin().lock().read_line(&mut input).unwrap();
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

macro_rules! xor_swap {
    ($x:expr, $y:expr) => {
        if $x != $y {
            $x ^= $y;
            $y ^= $x;
            $x ^= $y;
        }
    };
}

macro_rules! prefix_sum {
    ($vec:expr) => {
        $vec.iter()
            .scan(0, |sum, &x| {
                *sum += x;
                Some(*sum)
            })
            .collect::<Vec<i32>>()
    };
}

macro_rules! max {
    ($vec:expr) => {
        *$vec.iter().max().unwrap()
    };
}

macro_rules! sort_des {
    ($vec:expr) => {
        $vec.sort_unstable_by(|a, b| b.cmp(a));
    };
}

macro_rules! rout_vec {
    ($vec:expr) => {{
        for i in $vec {
            print!("{} ", i)
        }
        println!()
    }};
}

macro_rules! rout {
    ($x:expr) => {
        print!("{} ", $x)
    };
}

macro_rules! routln {
    ($x:expr) => {
        println!("{}", $x)
    };
}

/*~~~~~~~~~~~~~*
 * CODE BELOW: *
 *~~~~~~~~~~~~~*/
fn solve() {
    let mut w = rin!(String);
    let pref = &w[0..w.len() - 2];
    println!("{}i", pref)
}
fn main() {
    let t = rin!(usize);
    for _ in 1..=t {
        solve()
    }
}
