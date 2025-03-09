fn is_palindrome(n: Vec<char>) -> bool {
    let mut i = 0;
    let mut j = n.len() - 1;
    while i < j {
        if n[i] == n[j] {
            i += 1;
            j -= 1;
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_works() {
        let v = vec!['1', '1', '1'];
        assert_eq!(is_palindrome(v), true);
    }
}

fn main() {}
