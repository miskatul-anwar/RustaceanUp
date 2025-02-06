use std::collections::VecDeque;

fn is_palindrome(s1: &str) -> bool {
    let mut vec_deque = VecDeque::new();

    for c in s1.chars() {
        vec_deque.push_back(c);
    }

    let mut check = true;
    while vec_deque.len() > 1 {
        let head = vec_deque.pop_front().unwrap();
        let tail = vec_deque.pop_back().unwrap();
        if head != tail {
            check = false;
            break;
        }
    }

    check
}

fn main() {
    let s1 = "rustsur";
    if is_palindrome(s1) {
        println!("Is a palindrome!");
    } else {
        println!("Not a Palindrome");
    }
}
