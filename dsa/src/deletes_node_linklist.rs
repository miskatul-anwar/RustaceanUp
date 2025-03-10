#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn delete_nodes(head: Option<Box<ListNode>>, nums: Vec<i32>) -> Option<Box<ListNode>> {
    // Convert the array into a HashSet for O(1) lookups
    let nums_set: std::collections::HashSet<i32> = nums.into_iter().collect();

    // Create a dummy node to simplify edge cases (e.g., removing the head)
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut current = &mut dummy;

    // Iterate through the list
    while let Some(ref mut node) = current.next {
        if nums_set.contains(&node.val) {
            // Skip the node (remove it from the list)
            current.next = node.next.take();
        } else {
            // Move to the next node
            current = current.next.as_mut().unwrap();
        }
    }

    // Return the modified list (skipping the dummy node)
    dummy.next
}

// Helper function to create a linked list from a vector
fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in values.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

// Helper function to convert a linked list to a vector
fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;
    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }
    result
}

fn main() {
    // Example usage
    let head = create_list(vec![1, 2, 3, 4, 5]);
    let nums = vec![3, 5];
    let modified_head = delete_nodes(head, nums);
    println!("{:?}", list_to_vec(modified_head)); // Output: [1, 2, 4]
}
