fn main() {
    let n1: String = "Hello WOrld".to_string();
    let n: usize = n1.len();
    let sliced = &n1[0..=(n - n / 2) as usize];
    println!("{}", sliced);
    let numbers: Vec<i32> = (1..=50).collect();
    let sliced = &numbers[0..(n as usize)];
    for i in sliced {
        println!("{}", i);
    }
}
