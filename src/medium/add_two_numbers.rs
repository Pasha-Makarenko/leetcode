// 2. Add Two Numbers

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

  pub fn to_vec(&self) -> Vec<i32> {
    let mut vec = vec![self.val];
    let mut current = &self.next;

    while let Some(node) = current {
      vec.push(node.val);
      current = &node.next;
    }

    vec
  }
}

struct Solution;

impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;
    let mut first = l1;
    let mut second = l2;
    let mut carry = 0;

    while first.is_some() || second.is_some() || carry > 0 {
      let sum = carry
        + first.as_ref().map_or(0, |node| node.val)
        + second.as_ref().map_or(0, |node| node.val);

      carry = sum / 10;

      *current = Some(Box::new(ListNode::new(sum % 10)));
      current = &mut current.as_mut().unwrap().next;

      first = first.and_then(|node| node.next);
      second = second.and_then(|node| node.next);
    }

    head
  }
}

#[test]
fn test() {
  let test_cases = [
    (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
    (vec![0], vec![0], vec![0]),
    (vec![9, 9, 9, 9, 9, 9, 9], vec![9, 9, 9, 9], vec![8, 9, 9, 9, 0, 0, 0, 1]),
  ];

  for (l1, l2, expected) in test_cases {
    assert_eq!(
      Solution::add_two_numbers(ListNode::from_vec(l1), ListNode::from_vec(l2)).unwrap().to_vec(),
      expected
    );
  }
}
