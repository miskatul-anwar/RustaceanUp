#![allow(unused)]
use std::io::{stdin, stdout, BufWriter, Write};

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

const MAX_N: usize = 300010;

static mut STRIP: [char; MAX_N] = ['\0'; MAX_N];
static mut PENALTIES: [i32; MAX_N] = [0; MAX_N];

unsafe fn can_achieve(max_penalty: i32, n: usize, k: usize) -> bool {
    let mut segments = 0;
    let mut i = 0;

    while i < n {
        if STRIP[i] == 'R' && PENALTIES[i] > max_penalty {
            i += 1;
            continue;
        }

        let mut has_mandatory = false;
        while i < n && !(STRIP[i] == 'R' && PENALTIES[i] > max_penalty) {
            if STRIP[i] == 'B' && PENALTIES[i] > max_penalty {
                has_mandatory = true;
            }
            i += 1;
        }

        if has_mandatory {
            segments += 1;
        }
    }

    segments <= k
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let k: usize = sc.next();

        let strip_input: String = sc.next();
        unsafe {
            STRIP[..n].copy_from_slice(&strip_input.chars().collect::<Vec<char>>()[..n]);
            for i in 0..n {
                PENALTIES[i] = sc.next();
            }
        }

        let mut low = 0;
        let mut high = unsafe { PENALTIES[..n].iter().max().cloned().unwrap_or(0) };
        let mut answer = high;

        while low <= high {
            let mid = low + (high - low) / 2;
            unsafe {
                if can_achieve(mid, n, k) {
                    answer = mid;
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
        }

        writeln!(out, "{}", answer).unwrap();
    }
}
