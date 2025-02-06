fn bubble_sort(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}
fn main() {
    let mut arr = [64, 34, 25, 12, 22, 22, 11, 90];
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
