fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let diff = right - left;
    if diff % 2 == 0 && right % 2 == 0 && left % 2 == 0 {
        if diff <= 2 {
            return vec![-1, -1];
        } else {
            return vec![right + 1, left + 1];
        }
    }
    vec![-1, -1]
}
fn main() {}
