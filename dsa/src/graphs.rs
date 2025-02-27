#![allow(unused)]

use std::fmt::Debug;
type Node<T> = Option<Box<BinaryTree<T>>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BinaryTree<T> {
    key: T,
    left: Node<T>,
    right: Node<T>,
}

impl<T: Clone + Ord + ToString + Debug> BinaryTree<T> {
    fn new(root: T) -> Self {
        Self {
            key: root,
            left: None,
            right: None,
        }
    }

    fn insert_left(&mut self, key: T) {
        let mut node = BinaryTree::new(key);

        if self.left.is_none() {
            self.left = Some(Box::new(node))
        } else {
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    fn insert_right(&mut self, key: T) {
        let mut node = BinaryTree::new(key);

        if self.right.is_none() {
            self.right = Some(Box::new(node))
        } else {
            node.right = self.right.take();
            self.right = Some(Box::new(node))
        }
    }
}

fn main() {}
