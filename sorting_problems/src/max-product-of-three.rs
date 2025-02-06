pub fn maximum_product(nums: Vec<i32>) -> i32 {
    let mut sn: Vec<i32> = nums.into_iter().collect();
    let len = sn.len();
    sn.sort_unstable();

    let a = sn[len - 1] * sn[len - 2] * sn[len - 3];
    let b = sn[len - 1] * sn[0] * sn[1];

    a.max(b)
}
fn main() {}
