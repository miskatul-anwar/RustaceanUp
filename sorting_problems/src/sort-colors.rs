pub fn sort_colors(nums: &mut Vec<i32>) {
    if nums.len() < 1 {
        return;
    }

    let max = 1 + nums.iter().max().unwrap();
    let mut counter = vec![0; max as usize];

    for &v in nums.iter() {
        counter[v as usize] += 1;
    }

    let mut j = 0;
    for i in 0..max as usize {
        while counter[i] > 0 {
            nums[j] = i as i32;
            counter[i] -= 1;
            j += 1;
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    println!("Sorted: {:?}", nums);
}
