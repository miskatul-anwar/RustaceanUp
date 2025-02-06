fn insertion_sort(arr: &mut [usize]) {
    if arr.len() < 2 {
        println!("Can't be sorted!");
        return;
    }

    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i - 1;
        while j >= 0 as usize && arr[j] > key {
            arr[j + 1] = arr[j];
            j -= 1;
        }
        arr[j + 1] = key;
    }
}
fn main() {
    let mut arr = [1, 5, 3, 6, 2, 9];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}
