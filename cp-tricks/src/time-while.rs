use std::time::{Duration, Instant};

fn main() {
    let mut counter: i128 = 0;
    let time_now = Instant::now();
    let limit = Duration::new(5, 0);

    while Instant::now() - time_now < limit {
        counter += 1
    }

    println!("{}", counter)
}
