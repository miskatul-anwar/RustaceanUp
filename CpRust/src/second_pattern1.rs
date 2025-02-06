use std::io::{stdin, BufRead};

fn _rin() -> Vec<char> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.chars().collect()
}

fn pattern_match(t: Vec<char>, p: Vec<char>) {
    let mut q: Vec<String> = Vec::new();
    let mut s = "";
    let n = t.len();
    let m = p.len();

    let mut i: usize = 0;
    for i in 0..n {
        q.push(t[0..i].iter().collect());
    }

    // while s != p.concat() {}
}

fn main() {}
