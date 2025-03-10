struct List {
    size: usize,
    head: Option<Box<Node>>,
}

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl List {
    fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    fn push(&mut self, value: i32) {
        let node = Box::new(Node {
            data: value,
            next: self.head.take(),
        });

        self.head = Some(node);
        self.size += 1
    }

    fn pop(&mut self) -> Option<Box<i32>> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(Box::new(node.data))
    }

    fn peek(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.data)
    }
}
fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    if let Some(top) = list.peek() {
        println!("{}", top)
    }

    while let Some(node) = list.pop() {
        println!("{}", node)
    }
}
