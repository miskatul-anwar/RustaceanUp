use std::io::{stdin, BufRead};

fn main() {
    let mut input: String = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    for _ in 1..=t {
        input.clear();
        stdin().lock().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        input.clear();
        stdin().lock().read_line(&mut input).unwrap();
        let arr: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        let mut ans: Vec<i32> = Vec::new();
        ans.push(arr[0]);
        for i in 1..n {
            if arr[i - 1] <= arr[i] {
                ans.push(arr[i]);
            } else {
                ans.push(1);
                ans.push(arr[i]);
            }
        }
        println!("{}", ans.len());
        for i in ans {
            print!("{} ", i)
        }
        println!();
    }
}
