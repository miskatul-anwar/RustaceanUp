/*~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 *~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*/

#![allow(unused)]

use std::io::{stdin, BufRead};

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
    let (n, mut a) = (rin!(i64), rin_vec!(i64));

    a.sort_unstable();
    let (mut i, mut j, mut x, mut y) = (0, n - 1, 0_i64, 0_i64);
    let mut ok = false;

    while i <= j {
        if ok {
            x += a[i as usize];
            i += 1
        } else {
            y += a[j as usize];
            if j == 0 {
                break;
            }
            j -= 1
        }

        ok ^= true
    }

    routln!(x * x + y * y)
}
fn main() {
    solve()
}
