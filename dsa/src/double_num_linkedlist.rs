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

pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Step 1: Reverse the linked list
    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    let mut head = reverse(head);
    let mut carry = 0;
    let mut current = &mut head;

    // Step 2: Double each node's value and manage carry
    while let Some(node) = current {
        let new_val = node.val * 2 + carry;
        node.val = new_val % 10;
        carry = new_val / 10;
        current = &mut node.next;
    }

    // If there's leftover carry, add a new node
    if carry > 0 {
        *current = Some(Box::new(ListNode::new(carry)));
    }

    // Step 3: Reverse the list back
    reverse(head)
}

fn main() {}
