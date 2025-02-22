#![allow(unused)]
use std::{
    collections::{BTreeSet, HashSet},
    io::{stdin, stdout, BufWriter, Write},
};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

/*~~~~~~~~~~~~~*
 * CODE BELOW: *
 *~~~~~~~~~~~~~*/

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Pair<T: PartialEq + PartialOrd> {
    first: T,
    second: T,
}

impl<T: PartialEq + PartialOrd> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair {
            first: x,
            second: y,
        }
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();
    for _ in 1..=t {
        let (a, b) = (sc.next::<i32>(), sc.next::<i32>());
        let (xk, yk) = (sc.next::<i32>(), sc.next::<i32>());
        let (xq, yq) = (sc.next::<i32>(), sc.next::<i32>());

        let mut dirs: Vec<Pair<i32>> = vec![
            Pair::new(a, b),
            Pair::new(a, -b),
            Pair::new(-a, b),
            Pair::new(-a, -b),
            Pair::new(b, a),
            Pair::new(b, -a),
            Pair::new(-b, a),
            Pair::new(-b, -a),
        ];

        let mut st1 = HashSet::new();
        let mut st2 = HashSet::new();

        for d in dirs {
            let mut x = xk + d.first;
            let mut y = yk + d.second;
            st1.insert(Pair::new(x, y));

            x = xq + d.first;
            y = yq + d.second;

            st2.insert(Pair::new(x, y));
        }

        let mut ans = 0;

        for pos in st1 {
            if st2.iter().find(|x| **x == pos).is_some() {
                ans += 1
            }
        }

        writeln!(out, "{ans}");
    }
}
