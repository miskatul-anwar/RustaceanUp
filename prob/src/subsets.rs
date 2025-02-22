/*~~~~~~~~~~~~~*
 * CODE BELOW: *
 *~~~~~~~~~~~~~*/
// impl Solution {
//     pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         let mut result = vec![];
//         let mut subset = vec![];
//         Self::backtrack(&nums, 0, &mut subset, &mut result);
//         result
//     }

//     fn backtrack(nums: &Vec<i32>, index: usize, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
//         result.push(subset.clone()); // Store the current subset

//         for i in index..nums.len() {
//             subset.push(nums[i]); // Include nums[i]
//             Self::backtrack(nums, i + 1, subset, result);
//             subset.pop(); // Backtrack (remove last added element)
//         }
//     }
// }

fn main() {
    println!("Hello, World!")
}
