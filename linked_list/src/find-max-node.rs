#![allow(unused)]
use std::cmp::max;

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

struct List {
    size: usize,
    head: Link,
}

impl List {
    fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    fn push(&mut self, elem: i32) {
        let node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(node);
        self.size += 1
    }

    fn pop(&mut self) -> Option<i32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.elem)
    }

    fn max(&self) -> Option<i32> {
        let mut max_elem = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            max_elem = max_elem.max(node.elem);
            current = &node.next;
        }

        Some(max_elem)
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    println!("Max element: {}", list.max().unwrap())
}
