use text_io::read;
fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

fn balanced_par(s: &str) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        if c == '(' || c == '{' || c == '[' {
            stack.push(c);
        } else if let Some(top) = stack.pop() {
            if !par_match(top, c) {
                return false;
            }
        } else {
            return false;
        }
    }

    stack.is_empty()
}

fn main() {
    print!("Enter a string: ");
    let s1: String = read!();
    if balanced_par(&s1) {
        println!("Valid");
    } else {
        println!("Invalid");
    }
}
