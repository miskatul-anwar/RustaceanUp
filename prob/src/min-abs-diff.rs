use std::i32::MAX;
fn min_diff(ans: &mut Vec<Vec<i32>>, diff: &mut Vec<i32>, a: Vec<i32>) {
    for i in 1..a.len() {
        diff[i] = (a[i] - a[i - 1]).abs();
    }

    let min = *diff.iter().min().unwrap();

    for i in 1..diff.len() {
        if diff[i] == min {
            let mut temp = Vec::new();
            temp.push(a[i - 1]);
            temp.push(a[i]);
            ans.push(temp);
        }
    }
}

pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut a: Vec<i32> = arr.into_iter().collect();
    let mut diff = vec![MAX; a.len()];

    if !a.is_sorted() {
        a.sort_unstable();
        min_diff(&mut ans, &mut diff, a);
    } else {
        min_diff(&mut ans, &mut diff, a);
    }

    ans
}
fn main() {
    let arr = vec![2, 2, 1, 4, 4, 5];
    println!("{:?}", minimum_abs_difference(arr));
}
