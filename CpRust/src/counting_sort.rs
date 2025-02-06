fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 0 {
        return;
    }

    let max_bkt_num = 1 + nums.iter().max().unwrap();
    let mut counter = vec![0; max_bkt_num];

    for &v in nums.iter() {
        counter[v] += 1;
    }

    let mut j = 0;
    for i in 0..max_bkt_num {
        while counter[i] > 0 {
            nums[j] = i;
            counter[i] -= 1;
            j += 1;
        }
    }
}
fn main() {
    let mut nums = [2, 21, 12, 10, 5, 3, 9, 14];
    counting_sort(&mut nums);
    println!("{nums:?}");
}
