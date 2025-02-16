#![allow(unused)]

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]

struct Stack<T, const N: usize> {
    data: [Option<T>; N],
    size: usize,
}

impl<T, const N: usize> Stack<T, N> {
    fn new() -> Self {
        Stack {
            data: [(); N].map(|_| None),
            size: 0,
        }
    }

    fn push(&mut self, val: T) -> Result<(), &'static str> {
        if self.size < N {
            self.data[self.size] = Some(val);
            self.size += 1;
            Ok(())
        } else {
            Err("Stack Overflow!")
        }
    }

    fn pop(&mut self) -> Result<Option<T>, &'static str> {
        if self.size > 0 {
            self.size -= 1;
            Ok(self.data[self.size].take())
        } else {
            Err("Stack Underflow!")
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn peek(&self) -> Result<Option<&T>, &'static str> {
        if self.size > 0 {
            Ok(self.data[self.size - 1].as_ref())
        } else {
            Err("Empty Stack!")
        }
    }
}

fn main() {
    let mut st: Stack<char, 6> = Stack::new();
    let name = "miskat";

    for i in name.chars() {
        let res = st.push(i);
    }

    while let Some(top) = st.pop().unwrap() {
        println!("{:?}", st.peek())
    }
}
