use std::ffi::c_char;

unsafe extern "C" {
    fn scanf(format: *const c_char, ...) -> i32;
    fn printf(format: *const c_char, ...) -> i32;
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;

    unsafe {
        scanf(c"%d %d\n".as_ptr(), &mut num1, &mut num2);
        printf(c"You entered: %d\n".as_ptr(), sum(num1, num2));
    }
}

#[test]
fn sum_test() {
    assert_eq!(sum(2, 3), 5);
}
