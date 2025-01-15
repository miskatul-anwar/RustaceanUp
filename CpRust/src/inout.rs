use std::io::{self, BufRead};

struct Rin;

impl Rin {
    fn read_vector<T: std::str::FromStr>() -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        line.trim()
            .split_whitespace()
            .map(|x| x.parse::<T>().unwrap())
            .collect()
    }

    fn read_integer<T: std::str::FromStr>() -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        line.trim().parse::<T>().unwrap()
    }
}

fn main() {
    // Example usage:
    println!("Enter a vector of integers:");
    let values: Vec<i8> = Rin::read_vector();
    println!("You entered: {:?}", values);

    println!("Enter an integer:");
    let t: i8 = Rin::read_integer();
    println!("You entered: {}", t);
}
