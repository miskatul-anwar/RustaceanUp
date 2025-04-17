#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head;
    let mut prev = None;

    while let Some(mut node) = current {
        current = node.next.take();
        node.next = prev;
        prev = Some(node)
    }

    prev
}

fn main() {
    let third = Some(Box::new(ListNode { val: 3, next: None }));

    let second = Some(Box::new(ListNode {
        val: 2,
        next: third,
    }));

    let first = Some(Box::new(ListNode {
        val: 1,
        next: second,
    }));

    println!("{:?}", first);

    let new = reverse(first);

    println!("{:?}", new);
}
