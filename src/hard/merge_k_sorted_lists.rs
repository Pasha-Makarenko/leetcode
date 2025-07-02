// 23. Merge k Sorted Lists

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

#[derive(Debug)]
pub struct MaxHeap<T: Ord> {
  data: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
  pub fn new() -> Self {
    MaxHeap { data: Vec::new() }
  }

  pub fn push(&mut self, value: T) {
    self.data.push(value);
    let last_index = self.data.len() - 1;
    self.heapify_up(last_index);
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.data.is_empty() {
      return None;
    }

    let len = self.data.len();
    self.data.swap(0, len - 1);
    let min = self.data.pop();
    self.heapify_down(0);
    min
  }

  fn heapify_up(&mut self, index: usize) {
    if index == 0 {
      return;
    }

    let parent_index = (index - 1) / 2;
    if self.data[index] > self.data[parent_index] {
      self.data.swap(index, parent_index);
      self.heapify_up(parent_index);
    }
  }

  fn heapify_down(&mut self, index: usize) {
    let len = self.data.len();
    let left_child = 2 * index + 1;
    let right_child = 2 * index + 2;
    let mut smallest = index;

    if left_child < len && self.data[left_child] > self.data[smallest] {
      smallest = left_child;
    }
    if right_child < len && self.data[right_child] > self.data[smallest] {
      smallest = right_child;
    }

    if smallest != index {
      self.data.swap(index, smallest);
      self.heapify_down(smallest);
    }
  }
}

struct Solution;

impl Solution {
  pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap = MaxHeap::new();
    let mut result = None;

    for head in lists {
      let mut node = head;

      while let Some(entry) = node {
        heap.push(entry.val);
        node = entry.next;
      }
    }

    while let Some(value) = heap.pop() {
      let mut new_node = Box::new(ListNode::new(value));
      new_node.next = result;
      result = Some(new_node);
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [
    (
      vec![
        ListNode::from_vec(vec![1, 4, 5]),
        ListNode::from_vec(vec![1, 3, 4]),
        ListNode::from_vec(vec![2, 6]),
      ],
      vec![1, 1, 2, 3, 4, 4, 5, 6],
    ),
    (
      vec![
        ListNode::from_vec(vec![1, 3, 5]),
        ListNode::from_vec(vec![1, 3, 4]),
        ListNode::from_vec(vec![2, 6]),
        ListNode::from_vec(vec![7, 10]),
      ],
      vec![1, 1, 2, 3, 3, 4, 5, 6, 7, 10],
    ),
  ];

  for (lists, expected) in test_cases {
    assert_eq!(Solution::merge_k_lists(lists).unwrap().to_vec(), expected);
  }
}
