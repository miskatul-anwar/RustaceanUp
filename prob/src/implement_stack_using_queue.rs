use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Stack {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            queue1: VecDeque::new(),
            queue2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue1.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        while self.queue1.len() > 1 {
            self.queue2.push_back(self.queue1.pop_front().unwrap());
        }

        let top = self.queue1.pop_front().unwrap();
        std::mem::swap(&mut self.queue1, &mut self.queue2);

        top
    }

    fn top(&mut self) -> i32 {
        while self.queue1.len() > 1 {
            self.queue2.push_back(self.queue1.pop_front().unwrap());
        }

        let top = *self.queue1.front().unwrap();
        self.queue2.push_back(self.queue1.pop_front().unwrap());
        std::mem::swap(&mut self.queue1, &mut self.queue2);

        top
    }

    fn empty(&self) -> bool {
        self.queue1.is_empty() && self.queue2.is_empty()
    }
}

fn main() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    println!("{}", s.top()); // Output: 3
    s.pop();
    println!("{}", s.pop()); // Output: 2
    println!("{}", s.empty()); // Output: false
}
