fn main() {
    let arr: [u8; 4 as usize] = [10, 20, 30, 40];
    let mut i: usize = 0;
    while i < 4 {
        print!("{} ", arr[i]);
        i += 1;
    }
}
