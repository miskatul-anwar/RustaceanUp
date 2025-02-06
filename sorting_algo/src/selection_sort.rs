fn selection_sort(nums: &mut Vec<i32>) {
    let mut left = nums.len() - 1; // left marker
    while left > 0 {
        let mut pos_max = 0;
        for i in 1..=left {
            if nums[i] > nums[pos_max] {
                pos_max = i; // current loop's max value
            }
        }
        nums.swap(left, pos_max);
        left -= 1; // unsorted elements reduced by 1
    }
}

fn main() {
    let mut nums = vec![54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    selection_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
    // sorted nums: [18, 21, 22, 31, 32, 43, 54, 56, 75, 99]
}
