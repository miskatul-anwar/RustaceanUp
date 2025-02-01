// 2+1+7+4+5=?
// => recursive approach >sum = ((((2 + 1) + 7) + 4) + 5)

fn num_sum1(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        nums[0] + num_sum1(&nums[1..])
    }
}

fn main() {
    let nums = [2, 1, 7, 4, 5];
    println!("{}", num_sum1(&nums));
}
