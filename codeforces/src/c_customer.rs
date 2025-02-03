use std::collections::VecDeque;

fn rin_i32() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn rin_vec_i32() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn bfs(
    m: usize,
    adj: &Vec<Vec<usize>>,
    pair_u: &mut Vec<i32>,
    pair_v: &mut Vec<i32>,
    dist: &mut Vec<i32>,
) -> bool {
    dist.fill(-1);
    let mut queue = VecDeque::new();
    for u in 0..m {
        if pair_u[u] == -1 {
            dist[u] = 0;
            queue.push_back(u);
        }
    }
    let mut found = false;
    while let Some(u) = queue.pop_front() {
        for &v in &adj[u] {
            if pair_v[v] == -1 {
                found = true;
            } else if dist[pair_v[v] as usize] == -1 {
                dist[pair_v[v] as usize] = dist[u] + 1;
                queue.push_back(pair_v[v] as usize);
            }
        }
    }
    found
}

fn dfs(
    u: usize,
    adj: &Vec<Vec<usize>>,
    pair_u: &mut Vec<i32>,
    pair_v: &mut Vec<i32>,
    dist: &mut Vec<i32>,
) -> bool {
    for &v in &adj[u] {
        if pair_v[v] == -1
            || (dist[pair_v[v] as usize] == dist[u] + 1
                && dfs(pair_v[v] as usize, adj, pair_u, pair_v, dist))
        {
            pair_u[u] = v as i32;
            pair_v[v] = u as i32;
            return true;
        }
    }
    dist[u] = -1;
    false
}

fn max_matching(
    m: usize,
    _n: usize,
    adj: &Vec<Vec<usize>>,
    pair_u: &mut Vec<i32>,
    pair_v: &mut Vec<i32>,
    dist: &mut Vec<i32>,
) -> i32 {
    let mut result = 0;
    while bfs(m, adj, pair_u, pair_v, dist) {
        for u in 0..m {
            if pair_u[u] == -1 && dfs(u, adj, pair_u, pair_v, dist) {
                result += 1;
            }
        }
    }
    result
}

fn solve() {
    let n = rin_i32() as usize;
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        a[i] = rin_vec_i32();
    }

    let mut rev_prefix = vec![vec![0; n + 1]; n];
    for i in 0..n {
        for v in 1..=n {
            rev_prefix[i][v] = rev_prefix[i][v - 1] + a[i][n - v];
        }
    }

    let mut queues_for_v: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let mut possible_v = vec![false; n + 1];
    for v in 0..=n {
        possible_v[v] = false;
        for i in 0..n {
            if rev_prefix[i][v] == v as i32 {
                queues_for_v[v].push(i);
                possible_v[v] = true;
            }
        }
    }

    let mut answer = 0;
    for m in (0..=n).rev() {
        if m == 0 {
            answer = 0;
            break;
        }
        let mut valid = true;
        for v in 0..m {
            if v > n || !possible_v[v] {
                valid = false;
                break;
            }
        }
        if !valid {
            continue;
        }

        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); m];
        for v in 0..m {
            let queues = &queues_for_v[v];
            for &q in queues {
                adj[v].push(q);
            }
        }

        let mut pair_u = vec![-1; m];
        let mut pair_v = vec![-1; n];
        let mut dist = vec![-1; m];
        let match_count = max_matching(m, n, &adj, &mut pair_u, &mut pair_v, &mut dist);

        if match_count == m as i32 {
            answer = m;
            break;
        }
    }

    println!("{}", answer);
}

fn main() {
    let t = rin_i32();
    for _ in 0..t {
        solve();
    }
}
