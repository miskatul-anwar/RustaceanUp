#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

fn main() {
    let third = Some(Box::new(ListNode::new(3, None)));
    let second = Some(Box::new(ListNode::new(2, third)));
    let first = Some(Box::new(ListNode::new(1, second)));

    let mut current = first;

    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = node.next
    }

    println!("None")
}
