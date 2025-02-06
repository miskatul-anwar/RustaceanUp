fn counting_sort(nums: &mut [usize]) {
    let max = nums.iter().max().unwrap() + 1;
    let mut count = vec![0; max];

    for &i in nums.iter() {
        count[i] += 1;
    }

    let mut j = 0;
    for i in 0..max {
        while count[i] > 0 {
            nums[j] = i;
            count[i] -= 1;
            j += 1;
        }
    }
}

fn main() {
    let mut nums = [1, 5, 3, 6, 3, 2, 9];
    counting_sort(&mut nums);
    println!("{:?}", nums);
}
