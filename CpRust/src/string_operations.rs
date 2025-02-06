use std::{fmt, io::stdin};

fn _rin() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().chars().collect()
}

struct Str {
    data: Vec<char>,
}

impl Str {
    fn new() -> Self {
        Str { data: Vec::new() }
    }

    fn len(&mut self) -> usize {
        let mut i = 0;
        for _ in &self.data {
            i += 1;
        }
        i
    }

    fn subs(&self, start: usize, end: usize) -> Vec<char> {
        let mut sub: Vec<char> = Vec::new();
        for i in start..end {
            sub.push(self.data[i]);
        }
        sub
    }

    fn replace(&mut self, target: Vec<char>, rep: Vec<char>) {
        let n = self.data.len();
        let t_len = target.len();
        let r_len = rep.len();

        let mut i = 0;
        while i <= n - t_len {
            if self.data[i..i + t_len] == target[..] {
                self.data.splice(i..i + t_len, rep.iter().cloned());
                i += r_len;
            } else {
                i += 1;
            }
        }
    }

    fn get_value(&self, index: usize) -> char {
        self.data[index]
    }

    fn rep_index(&mut self, index: usize, tar: char) {
        self.data[index] = tar;
    }

    fn remove(&mut self, tar: char) {
        let mut new: Vec<char> = Vec::new();
        for ch in 0..self.data.len() {
            if self.data[ch] != tar {
                new.push(self.data[ch]);
            }
        }
        self.data = new;
    }

    fn insert(&mut self, tar: Vec<char>) {
        let size = self.data.len();
        let add_size = tar.len();
        let new_size = size + add_size;
        let mut new: Vec<char> = vec![' '; new_size];

        for i in 0..self.data.len() {
            new[i] = self.data[i];
        }

        for i in size..new_size {
            new[i] = tar[i - size];
        }

        self.data = new;
    }

    fn show(&self) -> Vec<char> {
        self.data.clone()
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ch in &self.data {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }
}

fn main() {
    let mut my_string = Str::new();
    my_string.insert("Hello".chars().collect());

    println!("{}", my_string);
}
