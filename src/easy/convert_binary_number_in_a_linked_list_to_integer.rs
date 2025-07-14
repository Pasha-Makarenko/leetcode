// 1290. Convert Binary Number in a Linked List to Integer

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

  pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &val in vec.iter() {
      *current = Some(Box::new(ListNode::new(val)));
      current = &mut current.as_mut().unwrap().next;
    }

    head
  }
}

struct Solution;

impl Solution {
  pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut node = head;
    let mut result = 0;

    while let Some(entry) = node {
      result = (result << 1) | entry.val;
      node = entry.next;
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [(vec![1, 0, 1], 5), (vec![0], 0)];

  for (head, expected) in test_cases {
    assert_eq!(Solution::get_decimal_value(ListNode::from_vec(head)), expected);
  }
}
