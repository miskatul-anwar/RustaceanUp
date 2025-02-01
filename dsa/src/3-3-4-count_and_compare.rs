// Checking if the given strings are anagrams

fn anagram_sol(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut c1 = [0; 26];
    let mut c2 = [0; 26];

    for c in s1.chars() {
        c1[(c as usize) - 97] += 1;
    }

    for c in s2.chars() {
        c2[(c as usize) - 97] += 1;
    }

    let mut pos = 0;
    let mut is_anagram = true;

    while pos < 26 && is_anagram {
        if c1[pos] != c2[pos] {
            is_anagram = false;
        }
        pos += 1;
    }

    is_anagram
}

fn main() {
    let s1 = "rust";
    let s2 = "trus";
    if anagram_sol(s1, s2) {
        println!("Is anagram!!");
    }
}
