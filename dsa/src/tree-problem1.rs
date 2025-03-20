use std::io::{self, BufRead};
struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn dfs(
    node: usize,
    parent: usize,
    tree: &Vec<Vec<usize>>,
    subtree_sum: &mut Vec<usize>,
    even_node: &mut Vec<usize>,
) {
    subtree_sum[node] = node;
    if node % 2 == 0 {
        even_node[node] = 1;
    }

    for &child in &tree[node] {
        if child == parent {
            continue;
        }

        dfs(child, node, tree, subtree_sum, even_node);

        // Add child's data to parent's subtree sum
        subtree_sum[node] += subtree_sum[child];
        even_node[node] += even_node[child];
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner::new(stdin.lock());

    let n: usize = sc.next();
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut subtree_sum: Vec<usize> = vec![0; n + 1];
    let mut even_node: Vec<usize> = vec![0; n + 1];

    println!("\nEnter the edges:");
    for _ in 0..n - 1 {
        let x: usize = sc.next();
        let y: usize = sc.next();

        tree[x].push(y);
        tree[y].push(x);
    }

    let root: usize = sc.next();

    dfs(root, usize::MAX, &tree, &mut subtree_sum, &mut even_node);

    let query_node: usize = sc.next();

    println!(
        "Subtree sum: {}\nEven Node Count: {}",
        subtree_sum[query_node], even_node[query_node]
    );
}
