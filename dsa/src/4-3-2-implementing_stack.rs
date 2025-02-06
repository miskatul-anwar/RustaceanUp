struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            size: 0,
            data: Vec::new(),
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
        self.size += 1;
    }

    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    fn pop(&mut self) -> Option<T> {
        if self.size != 0 {
            self.size -= 1;
            return self.data.pop();
        }
        None
    }

    fn peek(&self) -> Option<&T> {
        if self.size != 0 {
            return self.data.last();
        }
        None
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);

    while !stack.is_empty() {
        if let Some(val) = stack.pop() {
            print!("{val} ");
        }
    }

    stack.clear();
}
