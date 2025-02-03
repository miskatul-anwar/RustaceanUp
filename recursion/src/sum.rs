fn sum(nums: &[usize], idx: usize) -> usize {
    if idx == 0 {
        return nums[idx];
    }
    nums[idx] + sum(&nums, idx - 1)
}

fn factorial(num: usize) -> usize {
    if num == 1 {
        return num;
    }

    num * factorial(num - 1)
}
fn main() {
    let nums = [1, 2, 3, 4, 5];
    let idx = nums.len() - 1;

    println!("{}", sum(&nums, idx));
    println!("{}", factorial(idx));
}
