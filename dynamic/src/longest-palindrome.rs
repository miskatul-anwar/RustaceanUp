fn check_palindrome(sub: &str) -> bool {
    let mut i = 0;
    let mut j = sub.len() - 1;

    while i < j {
        if sub.chars().nth(i).unwrap() != sub.chars().nth(j).unwrap() {
            return false;
        }

        i += 1;
        j -= 1;
    }

    true
}

fn longest_palindrome<'a>(s: &'a str) -> &'a str {
    let mut longest = "";
    let mut max_len = 0;

    for i in 0..s.len() {
        for j in i..s.len() {
            let sub = &s[i..=j];

            if check_palindrome(sub) && sub.len() > max_len {
                longest = sub;
                max_len = sub.len();
            }
        }
    }

    longest
}

fn main() {
    let s = "forgeeksskeegfor";
    println!("{}", longest_palindrome(s));
}
