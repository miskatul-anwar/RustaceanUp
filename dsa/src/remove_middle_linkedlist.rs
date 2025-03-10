#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Step 1: Calculate the size of the list
    let mut size = 0;
    let mut current = &head;
    while let Some(node) = current {
        current = &node.next;
        size += 1;
    }

    // Step 2: Handle edge cases
    if size <= 1 {
        return None; // If the list is empty or has only one node, return None
    }

    // Step 3: Find the middle node and remove it
    let middle_index = size / 2;
    let mut dummy = Box::new(ListNode { val: 0, next: head }); // Dummy node to simplify edge cases
    let mut current = &mut dummy;

    for _ in 0..middle_index {
        current = current.next.as_mut().unwrap(); // Move to the node before the middle node
    }

    // Remove the middle node
    let next_node = current.next.as_mut().unwrap().next.take();
    current.next = next_node;

    // Return the modified list (skipping the dummy node)
    dummy.next
}

// Helper function to create a linked list from a vector

fn main() {}
