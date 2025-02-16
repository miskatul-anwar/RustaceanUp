pub fn trim_mean(arr: Vec<i32>) -> f64 {
    let mut a: Vec<i32> = arr.into_iter().collect();
    a.sort_unstable();
    let (mut len, mut sum, offset) = (0, 0, a.len() / 20);

    for i in offset..a.len() - offset {
        sum += a[i];
        len += 1;
    }

    (sum as f64) / (len as f64)
}
fn main() {
    let arr = vec![
        6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5, 10, 8,
        6, 6, 1, 0, 6, 10, 8, 2, 3, 4,
    ];
    println!("{}", trim_mean(arr));
}
