#![allow(unused)]
struct MyQueue {
    push_stack: Vec<i32>,
    pop_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            push_stack: Vec::new(),
            pop_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.push_stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.pop_stack.is_empty() {
            while let Some(val) = self.push_stack.pop() {
                self.pop_stack.push(val);
            }
        }
        self.pop_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.pop_stack.is_empty() {
            while let Some(val) = self.push_stack.pop() {
                self.pop_stack.push(val);
            }
        }
        *self.pop_stack.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.push_stack.is_empty() && self.pop_stack.is_empty()
    }
}

fn main() {
    let mut q = MyQueue::new();
    q.push(2);
    let v = q.pop();
    let b = q.empty();
    let p = q.peek();
}
