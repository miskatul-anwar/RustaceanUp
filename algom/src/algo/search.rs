pub fn binary_search<T: Ord>(arr: &[T], target: T) -> Result<usize, &'static str> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if &arr[mid] == &target {
            return Ok(mid);
        } else if &arr[mid] < &target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    Err("Index not found")
}

pub fn linear_search<T: PartialEq>(arr: &[T], target: T) -> Result<usize, &'static str> {
    for (index, element) in arr.iter().enumerate() {
        if element == &target {
            return Ok(index);
        }
    }
    Err("Index not found")
}

// fn main() {
//     let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     match binary_search(&arr, 5) {
//         Ok(index) => println!("Element found at index: {}", index),
//         Err(err) => println!("{}", err),
//     }
//     match linear_search(&arr, 5) {
//         Ok(index) => println!("Element found at index: {}", index),
//         Err(err) => println!("{}", err),
//     }
// }
