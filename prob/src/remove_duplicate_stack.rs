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

/*~~~~~~~~~~~~~*
 * CODE BELOW: *
 *~~~~~~~~~~~~~*/
use std::collections::{HashMap, HashSet};

fn remove_duplicate_letters(s: String) -> String {
    let mut char_count = HashMap::new();
    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    // Step 1: Count frequency of each character
    for c in s.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    // Step 2: Iterate through characters
    for c in s.chars() {
        // Decrease the count for current character
        *char_count.get_mut(&c).unwrap() -= 1;

        // If already in stack, skip it
        if visited.contains(&c) {
            continue;
        }

        // Step 3: Remove elements from stack if conditions are met
        while let Some(&top) = stack.last() {
            if top > c && *char_count.get(&top).unwrap() > 0 {
                stack.pop();
                visited.remove(&top);
            } else {
                break;
            }
        }

        // Step 4: Add current character to stack
        stack.push(c);
        visited.insert(c);
    }

    // Convert stack to string
    stack.into_iter().collect()
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let s = sc.next::<String>();

    writeln!(out, "{}", remove_duplicate_letters(s)).unwrap()
}
