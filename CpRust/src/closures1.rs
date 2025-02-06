fn main() {
    let mut name: Vec<char> = vec!['A', 'B', 'C', 'D'];

    name = name.iter().flat_map(|i| i.to_lowercase()).collect();
    name.iter().map(|i| print!("{i:?} ")).for_each(drop);
    println!();

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let sum: i32 = numbers.iter().sum();

    println!("{sum}");

    numbers = numbers.iter().map(|i| i + 1).collect();
    numbers.iter().map(|i| print!("{i:?} ")).for_each(drop);
}
