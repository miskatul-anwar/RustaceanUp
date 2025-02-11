use algom::algo::sort::bucket_sort;

fn main() {
    println!("Hello, world!");
    let mut v = vec![5, 2, 3];
    bucket_sort(&mut v);
    v.iter().for_each(|i| print!("{} ", i));
}
