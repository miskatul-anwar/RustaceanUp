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
pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let original_color = image[sr as usize][sc as usize];
    if original_color == color {
        return image;
    }

    fn dfs(image: &mut Vec<Vec<i32>>, r: usize, c: usize, original_color: i32, new_color: i32) {
        let rows = image.len();
        let cols = image[0].len();

        if r >= rows || c >= cols || image[r][c] != original_color {
            return;
        }

        image[r][c] = new_color;

        if r > 0 {
            dfs(image, r - 1, c, original_color, new_color);
        }
        if r + 1 < rows {
            dfs(image, r + 1, c, original_color, new_color);
        }
        if c > 0 {
            dfs(image, r, c - 1, original_color, new_color);
        }
        if c + 1 < cols {
            dfs(image, r, c + 1, original_color, new_color);
        }
    }

    dfs(&mut image, sr as usize, sc as usize, original_color, color);
    image
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();
    for _ in 1..=t {}
}
