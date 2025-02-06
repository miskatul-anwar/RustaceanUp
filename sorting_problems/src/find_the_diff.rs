pub fn find_the_difference(s: String, t: String) -> char {
    let mut count = vec![0; 26];

    for c in s.chars() {
        count[c as usize - 'a' as usize] += 1;
    }

    for c in t.chars() {
        if count[c as usize - 'a' as usize] == 0 {
            return c;
        }
        count[c as usize - 'a' as usize] -= 1;
    }
    ' '
}
fn main() {}
