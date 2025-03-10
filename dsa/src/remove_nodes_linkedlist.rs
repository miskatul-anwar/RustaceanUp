#![allow(unused)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Step 1: Reverse the linked list
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    // Step 2: Traverse the reversed list, keeping only max values
    let mut rev_head = prev;
    let mut max_seen = i32::MIN;
    let mut new_head = None;
    let mut new_tail = &mut new_head;

    while let Some(mut node) = rev_head {
        rev_head = node.next.take();
        if node.val >= max_seen {
            max_seen = node.val;
            *new_tail = Some(node);
            new_tail = &mut new_tail.as_mut().unwrap().next;
        }
    }

    // Step 3: Reverse again to restore order
    let mut prev = None;
    let mut current = new_head;

    while let Some(mut node) = current {
        current = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}
fn main() {}
