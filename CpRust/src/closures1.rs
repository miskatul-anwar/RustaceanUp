fn main() {
    let mut name: Vec<char> = vec!['A', 'B', 'C'];

    name = name.iter().flat_map(|i| i.to_lowercase()).collect();

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    numbers = numbers.iter().map(|i| i + 1).collect();

    println!("{:?}", name);
    println!("{:?}", numbers);
}
