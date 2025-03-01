#![allow(unused)]
pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut result = Vec::new();

    fn is_dividing(num: i32) -> bool {
        let mut num_copy = num;

        while num_copy > 0 {
            let digit = num_copy % 10;

            if digit == 0 || num % digit != 0 {
                return false;
            }

            num_copy /= 10
        }

        true
    }

    for i in left..=right {
        if is_dividing(i) {
            result.push(i);
        }
    }

    result
}
fn main() {
    let result = self_dividing_numbers(47, 85);

    for i in result {
        print!("{i} ")
    }

    println!()
}
