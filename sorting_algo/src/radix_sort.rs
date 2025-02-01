fn radix_sort(nums: &mut [usize]) {
    if nums.len() < 2 {
        return;
    }

    let max = *nums.iter().max().unwrap();
    let radix = nums.len().next_power_of_two();
    let mut digit = 1;

    while digit <= max {
        let index_of = |x| x / digit % radix;
        let mut counter = vec![0; radix];

        for &x in nums.iter() {
            counter[index_of(x)] += 1;
        }

        for i in 1..radix {
            counter[i] += counter[i - 1];
        }

        for &x in nums.to_owned().iter().rev() {
            counter[index_of(x)] -= 1;
            nums[counter[index_of(x)]] = x;
        }

        digit *= radix;
    }
}

fn main() {
    let mut nums = [170, 45, 75, 90, 802, 24, 2, 66];
    radix_sort(&mut nums);
    println!("{:?}", nums);
}
