/****************************************
 *       THIS ALGORITHMS IS USED        *
 * FOR FINDING THE MAXIMUM SUBARRAY SUM *
 *      IN LINEAR TIME COMPLEXITY       *
 ****************************************/
fn main() {
    let nums = [4, -5, 3, -2, 1, 5, -6, 3];
    let mut max_sum = 0;
    let mut curr_sum = 0;

    for num in nums {
        curr_sum += num;
        max_sum = max_sum.max(curr_sum);
        curr_sum = curr_sum.max(0);
    }

    println!("{max_sum}");
}
