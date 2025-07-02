// 25. Reverse Nodes in k-Group

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
  pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
      return head;
    }

    let mut head = head;
    let mut main_head = None;
    let mut prev_tail = &mut main_head;

    while head.is_some() {
      let mut i = 0;
      let mut pointer = &head;

      while i < k && pointer.is_some() {
        pointer = &pointer.as_ref().unwrap().next;
        i += 1;
      }

      if i < k {
        *prev_tail = head.take();
        break;
      }

      let mut reversed_head = None;
      let mut current = head.take();

      for _ in 0..k {
        if let Some(mut node) = current {
          current = node.next.take();
          node.next = reversed_head.take();
          reversed_head = Some(node);
        }
      }

      *prev_tail = reversed_head;

      let mut current_link_node = prev_tail.as_mut().unwrap();

      for _ in 0..(k - 1) {
        current_link_node = current_link_node.next.as_mut().unwrap();
      }

      prev_tail = &mut current_link_node.next;

      head = current;
    }

    main_head
  }
}

#[test]
fn test() {
  let test_cases = [
    (ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2, vec![2, 1, 4, 3, 5]),
    (ListNode::from_vec(vec![1, 2, 3, 4, 5]), 3, vec![3, 2, 1, 4, 5]),
  ];

  for (head, k, expected) in test_cases {
    assert_eq!(Solution::reverse_k_group(head, k).unwrap().to_vec(), expected);
  }
}
