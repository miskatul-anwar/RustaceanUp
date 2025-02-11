use std::cmp::Ord;
//------------------ inserstion sort
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
//-------------------------- bubble sort
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
//----------------------radix sort
pub fn counting_sort_for_radix(arr: &mut [i32], place: i32) {
    let len = arr.len();
    let mut output = vec![0; len];
    let mut count = vec![0; 10];

    for &num in arr.iter() {
        let index = (num / place) % 10;
        count[index as usize] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        let index = (num / place) % 10;
        output[count[index as usize] as usize - 1] = num;
        count[index as usize] -= 1;
    }

    for i in 0..len {
        arr[i] = output[i];
    }
}

pub fn radix_sort(arr: &mut [i32]) {
    let max = match arr.iter().max() {
        Some(&max) => max,
        None => return,
    };

    let mut place = 1;
    while max / place > 0 {
        counting_sort_for_radix(arr, place);
        place *= 10;
    }
}
// -------------------------- counting sort
pub fn counting_sort(arr: &mut [usize]) {
    let max_value = *arr.iter().max().unwrap_or(&0);
    let mut count = vec![0; max_value + 1];
    let mut output = vec![0; arr.len()];

    for &num in arr.iter() {
        count[num] += 1;
    }

    for i in 1..=max_value {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        output[count[num] - 1] = num;
        count[num] -= 1;
    }

    for i in 0..arr.len() {
        arr[i] = output[i];
    }
}
// ------------------------------ quick sort
fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] <= arr[pivot] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, pivot);
    i + 1
}

pub fn quicksort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let pi = partition(arr, low, high);
        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}
// ---------------------------------- merge sort
fn merge<T: Ord + Copy>(left: &[T], right: &[T], merged: &mut [T]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut merged_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            merged[merged_index] = left[left_index];
            left_index += 1;
        } else {
            merged[merged_index] = right[right_index];
            right_index += 1;
        }
        merged_index += 1;
    }

    while left_index < left.len() {
        merged[merged_index] = left[left_index];
        left_index += 1;
        merged_index += 1;
    }

    while right_index < right.len() {
        merged[merged_index] = right[right_index];
        right_index += 1;
        merged_index += 1;
    }
}

pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        let mid = len / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        let mut merged = arr.to_vec();
        merge(&arr[..mid], &arr[mid..], &mut merged[..]);
        arr.copy_from_slice(&merged);
    }
}
// -------------------------------- heapsort
fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    // Build the heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // Extract elements from the heap one by one
    for i in (0..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}
// -------------------bucket sort
pub fn bucket_sort<T: Ord + Copy>(arr: &mut [T])
where
    T: Into<f64> + Copy,
{
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Create empty buckets
    let mut buckets: Vec<Vec<T>> = vec![Vec::new(); n];

    // Determine the min and max values to normalize the array elements
    let min_value: T = *arr.iter().min().unwrap();
    let max_value: T = *arr.iter().max().unwrap();

    // Calculate the range and scale factors to normalize elements to the range [0, n)
    let range = max_value.into() - min_value.into();

    // Put array elements into different buckets
    for &num in arr.iter() {
        let normalized = (num.into() - min_value.into()) / range;
        let index = ((normalized * (n as f64)) as usize).min(n - 1);
        buckets[index].push(num);
    }

    // Sort individual buckets
    for bucket in buckets.iter_mut() {
        bucket.sort();
    }

    // Concatenate all buckets into arr[]
    let mut idx = 0;
    for bucket in buckets {
        for &num in bucket.iter() {
            arr[idx] = num;
            idx += 1;
        }
    }
}
