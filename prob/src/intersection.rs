use std::collections::HashSet;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = nums1.into_iter().collect();
    let set2: HashSet<i32> = nums2.into_iter().collect();

    set1.intersection(&set2).cloned().collect()
}
fn main() {}
