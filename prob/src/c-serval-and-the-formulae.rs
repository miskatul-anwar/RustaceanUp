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

#[derive(Clone, Copy)]
struct State {
    valid: bool,
    prev_i: i32,
    prev_c1: i32,
    prev_c2: i32,
    bit_val: i32,
}

impl Default for State {
    fn default() -> Self {
        State {
            valid: false,
            prev_i: -1,
            prev_c1: -1,
            prev_c2: -1,
            bit_val: -1,
        }
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let test_cases: i32 = sc.next();
    for _ in 0..test_cases {
        let x: i64 = sc.next();
        let y: i64 = sc.next();

        if x == y {
            writeln!(out, "-1").unwrap();
            continue;
        }

        const L: i32 = 62;
        let mut dp = vec![vec![vec![State::default(); 2]; 2]; (L + 1) as usize];

        dp[0][0][0].valid = true;

        for i in 0..L {
            let bx = ((x >> i) & 1) as i32;
            let by = ((y >> i) & 1) as i32;
            for c1 in 0..2 {
                for c2 in 0..2 {
                    if !dp[i as usize][c1 as usize][c2 as usize].valid {
                        continue;
                    }
                    for bit in 0..2 {
                        let s1 = bx + bit + c1;
                        let s2 = by + bit + c2;
                        let a_bit = s1 & 1;
                        let b_bit = s2 & 1;
                        if a_bit == 1 && b_bit == 1 {
                            continue;
                        }
                        let nc1 = s1 >> 1;
                        let nc2 = s2 >> 1;
                        if !dp[(i + 1) as usize][nc1 as usize][nc2 as usize].valid {
                            dp[(i + 1) as usize][nc1 as usize][nc2 as usize] = State {
                                valid: true,
                                prev_i: i,
                                prev_c1: c1,
                                prev_c2: c2,
                                bit_val: bit,
                            };
                        }
                    }
                }
            }
        }

        let mut final_state_found = false;
        let mut final_c1 = -1;
        let mut final_c2 = -1;
        'outer: for c1 in 0..2 {
            for c2 in 0..2 {
                if dp[L as usize][c1 as usize][c2 as usize].valid {
                    if c1 == 1 && c2 == 1 {
                        continue;
                    }
                    final_state_found = true;
                    final_c1 = c1;
                    final_c2 = c2;
                    break 'outer;
                }
            }
        }

        if !final_state_found {
            writeln!(out, "-1").unwrap();
            continue;
        }

        let mut k: i64 = 0;
        let mut i = L;
        let mut cur_c1 = final_c1;
        let mut cur_c2 = final_c2;
        while i > 0 {
            let st = dp[i as usize][cur_c1 as usize][cur_c2 as usize];
            let prev_i = st.prev_i;
            let prev_c1 = st.prev_c1;
            let prev_c2 = st.prev_c2;
            let bit = st.bit_val;
            if bit == 1 {
                k |= 1 << (i - 1);
            }
            i = prev_i;
            cur_c1 = prev_c1;
            cur_c2 = prev_c2;
        }

        if k > 1_000_000_000_000_000_000 {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{}", k).unwrap();
        }
    }
}
