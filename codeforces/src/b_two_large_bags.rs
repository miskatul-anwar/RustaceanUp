/*~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 *~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*/

#![allow(unused)]

use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

macro_rules! read_line {
    () => {{
        let mut input = String::new();
        stdin().lock().read_line(&mut input).unwrap();
        input
    }};
}

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

macro_rules! rin_vec_char {
    () => {{
        let mut input = String::new();
        stdin().lock().read_line(&mut input).unwrap();
        input.trim().chars().collect::<Vec<char>>()
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

macro_rules! min {
    (vec:expr) => {
        *vec.iter().min().unwrap()
    };
}

macro_rules! sort_des {
    ($vec:expr) => {
        $vec.sort_unstable_by(|a, b| b.cmp(a));
    };
}

macro_rules! vec_to_string {
    ($vec:expr) => {
        $vec.iter().collect::<String>()
    };
}

macro_rules! string_to_vec {
    ($string:expr) => {
        $string.chars().collect::<Vec<char>>()
    };
}

macro_rules! stot {
    ($type:ty, $value:expr) => {
        $value.parse::<$type>().expect("Conversion failed")
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
    let n = rin!(usize);
    let mut a = rin_vec!(i32);
    let mut mp = HashMap::new();

    for (k, v) in a.iter().enumerate() {
        mp.insert(k, v);
    }

    let mut flag = 1;
}
fn main() {
    let t = rin!(usize);
    for _ in 1..=t {
        solve()
    }
}
