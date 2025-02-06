// pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
//     let len = nums.len();
//     let mut counts = vec![0; len];

//     for i in 0..len {
//         let mut count = 0;
//         for j in 0..len {
//             if nums[i] > nums[j] {
//                 count += 1;
//             }
//         }
//         counts[i] = count;
//     }
//     counts
// }
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let (mut bucket, len) = (vec![0; 102], nums.len());
    let mut nums: Vec<i32> = nums.into_iter().collect();

    for i in 0..len {
        bucket[nums[i] as usize + 1] += 1;
    }

    for i in 1..102 as usize {
        bucket[i] += bucket[i - 1];
    }

    for i in 0..len {
        nums[i] = bucket[nums[i] as usize];
    }

    nums
}
fn main() {
    let nums = vec![8, 1, 2, 2, 3];
    println!("{:?}", smaller_numbers_than_current(nums));
}
