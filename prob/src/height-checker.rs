pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut expected: Vec<i32> = heights.clone();
    expected.sort_unstable();

    let mut wpos = 0;
    for (height, exp) in heights.iter().zip(expected.iter()) {
        if height != exp {
            wpos += 1;
        }
    }
    wpos
}
fn main() {}
