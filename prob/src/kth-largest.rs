use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut h_num: BinaryHeap<i32> = nums.into_iter().collect();

    while let Some(top) = h_num.pop() {
        if k == 1 {
            return top;
        }
        k -= 1;
    }
    1
}
fn main() {
    let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let k = 4;
    println!("{}", find_kth_largest(nums, k));
}
