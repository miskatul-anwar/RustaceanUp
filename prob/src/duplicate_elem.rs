use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let vec_size = nums.len();
    let set: HashSet<i32> = nums.into_iter().collect();

    let mut contains = false;
    if set.len() != vec_size {
        contains = true
    }

    contains
}
fn main() {}
