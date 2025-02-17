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
    ($vec:expr) => {
        *$vec.iter().min().unwrap()
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
        print!("\n")
    }};
}

macro_rules! is_even {
    ($num:expr) => {
        $num % 2 == 0
    };
}

macro_rules! is_odd {
    ($num:expr) => {
        $num % 2 != 0
    };
}

macro_rules! routln {
    ($x:expr) => {
        print!("{}\n", $x)
    };
}

macro_rules! nl {
    () => {
        print!("\n");
    };
}

/*~~~~~~~~~~~~~*
 * CODE BELOW: *
 *~~~~~~~~~~~~~*/

fn main() {
    let t = rin!(i32);
    for _ in 1..=t {
        let v = rin_vec!(i64);
        let (mut a, mut b, c, d) = (v[0], v[1], v[2], v[3]);

        let mut moves = 0;

        if a == c && b == d {
            routln!(0);
            continue;
        }

        if d < b {
            routln!(-1);
            continue;
        }

        while b != d {
            a += 1;
            b += 1;
            moves += 1
        }

        if a < c {
            routln!(-1);
            continue;
        }

        while a != c {
            a -= 1;
            moves += 1;
        }

        routln!(moves)
    }
}
