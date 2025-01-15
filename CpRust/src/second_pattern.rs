use std::{
    io::{stdin, BufRead},
    usize,
};

fn _rin() -> Vec<char> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.chars().collect()
}

fn table(p: Vec<char>) -> Vec<i32> {
    let m = p.len();
    let mut s: Vec<i32> = vec![0; m as usize];
    let mut j: usize = 0;
    let mut i: usize = 1;
    while i < m {
        if p[i] == p[j] {
            j += 1;
            s[i] = j as i32;
            i += 1;
        } else {
            if j != 0 {
                j = s[j - 1] as usize;
            } else {
                s[i as usize] = 0;
                i += 1;
            }
        }
    }
    s
}

/* NOTE: The Algorithm of second pattern matching */
fn second_matching(t: Vec<char>, p: Vec<char>) {
    let n = t.len();
    let m = p.len();
    let s = table(p.clone());

    let mut i = 0;
    let mut j = 0;

    while i < n && j != m {
        if p[j] == t[i] {
            i += 1;
            j += 1;
        }
        if j == m {
            println!("Pattern found at index: {}", i - j);
        } else if p[j] != t[i] {
            if j != 0 {
                j = s[j - 1] as usize;
            } else {
                i += 1;
            }
        }
    }
}

fn main() {
    let t = _rin();
    let p = _rin();
    second_matching(t, p);
}
