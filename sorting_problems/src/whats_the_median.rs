use std::{i64::MAX, io};

fn rin_i64() -> Option<i64> {
    let mut input = String::new();
    let stdin = io::stdin().read_line(&mut input);

    if stdin.is_err() {
        return None;
    }

    input.trim().parse::<i64>().ok()
}

fn main() {
    let mut nums = vec![MAX; 100001];
    let mut i = 0;

    while let Some(num) = rin_i64() {
        nums[i] = num;
        nums.sort_unstable();

        if i % 2 == 0 {
            println!("{}", nums[i / 2]);
        } else {
            let odd_med = (nums[i / 2] + nums[i / 2 + 1]) / 2;
            println!("{odd_med}");
        }

        i += 1;
    }
}
