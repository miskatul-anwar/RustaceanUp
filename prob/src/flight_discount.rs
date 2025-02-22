use std::cmp::Reverse;
use std::collections::BinaryHeap;
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

fn min_costs(from: usize, neighbors: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let n = neighbors.len();
    let mut min_costs = vec![i64::MAX; n];
    let mut pq = BinaryHeap::new();

    min_costs[from] = 0;
    pq.push(Reverse((0, from)));

    while let Some(Reverse((curr_cost, curr))) = pq.pop() {
        if curr_cost > min_costs[curr] {
            continue;
        }

        for &(neighbor, cost) in &neighbors[curr] {
            let new_cost = curr_cost + cost;
            if new_cost < min_costs[neighbor] {
                min_costs[neighbor] = new_cost;
                pq.push(Reverse((new_cost, neighbor)));
            }
        }
    }

    min_costs
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let city_num = sc.next::<usize>();
    let flight_num = sc.next::<usize>();

    let mut neighbors = vec![Vec::new(); city_num];
    let mut reverse_neighbors = vec![Vec::new(); city_num];

    for _ in 0..flight_num {
        let from = sc.next::<usize>() - 1;
        let to = sc.next::<usize>() - 1;
        let cost = sc.next::<i64>();

        neighbors[from].push((to, cost));
        reverse_neighbors[to].push((from, cost));
    }

    let start_costs = min_costs(0, &neighbors);
    let end_costs = min_costs(city_num - 1, &reverse_neighbors);

    let mut total_min = i64::MAX;

    for c in 0..city_num {
        for &(n, nc) in &neighbors[c] {
            if start_costs[c] == i64::MAX || end_costs[n] == i64::MAX {
                continue;
            }
            total_min = total_min.min(start_costs[c] + (nc / 2) + end_costs[n]);
        }
    }

    writeln!(out, "{}", total_min).unwrap();
}
