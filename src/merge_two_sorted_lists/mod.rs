// Definition for singly-linked list.
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
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;
    let mut l1 = list1;
    let mut l2 = list2;
    while l1.is_some() && l2.is_some() {
        let mut node = Box::new(ListNode::new(0));
        if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
            node.val = l1.as_ref().unwrap().val;
            l1 = l1.unwrap().next;
        } else {
            node.val = l2.as_ref().unwrap().val;
            l2 = l2.unwrap().next;
        }
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
    }
    if l1.is_some() {
        tail.next = l1;
    }
    if l2.is_some() {
        tail.next = l2;
    }
    head.next
}
