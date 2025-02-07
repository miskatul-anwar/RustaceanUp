use std::io::{self, Write};

fn main() {
    // Read input strings
    let (mut a, mut b, mut c) = (String::new(), String::new(), String::new());

    io::stdout().flush().unwrap();

    // Read input
    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();
    io::stdin().read_line(&mut c).unwrap();

    // Trim to remove newlines
    let a = a.trim();
    let b = b.trim();
    let c = c.trim();

    // Concatenate and sort
    let mut s: Vec<char> = (a.to_string() + b).chars().collect();
    let mut c_chars: Vec<char> = c.chars().collect();

    s.sort_by(|x, y| x.cmp(y));
    c_chars.sort_by(|x, y| x.cmp(y));

    // Check for equality and print result
    if s == c_chars {
        println!("YES");
    } else {
        println!("NO");
    }
}
