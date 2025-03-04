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
static mut SUBTREE_SUM: usize = 0;

fn dfs(node: usize, parent: usize, tree: &Vec<Vec<usize>>) {
    unsafe { SUBTREE_SUM += node }

    for &child in &tree[node] {
        if child == parent {
            continue;
        }

        dfs(child, node, tree);
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n: usize = sc.next();
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..n - 1 {
        // Fix: A tree with `n` nodes has `n-1` edges
        let x: usize = sc.next();
        let y: usize = sc.next();

        tree[x].push(y);
        tree[y].push(x);
    }

    let q: usize = sc.next();

    for _ in 0..q {
        let v: usize = sc.next();

        unsafe {
            SUBTREE_SUM = 0;
        }

        dfs(v, usize::MAX, &tree);

        let mut ans;
        unsafe {
            ans = SUBTREE_SUM;
        }

        writeln!(out, "{}", ans).unwrap()
    }
}
