// 3479. Fruits Into Baskets III

struct SegmentTree<T> {
  size: usize,
  data: Vec<T>,
  neutral: T,
  merge: fn(T, T) -> T,
}

impl<T: Copy> SegmentTree<T> {
  pub fn new(arr: &[T], neutral: T, merge: fn(T, T) -> T) -> Self {
    let size = arr.len();
    if size == 0 {
      return SegmentTree { size: 0, data: Vec::new(), neutral, merge };
    }

    let mut data = vec![neutral; 4 * size];
    Self::build(arr, &mut data, 0, 0, size - 1, merge);

    SegmentTree { size, data, neutral, merge }
  }

  fn build(arr: &[T], data: &mut Vec<T>, v: usize, tl: usize, tr: usize, merge: fn(T, T) -> T) {
    if tl == tr {
      data[v] = arr[tl];
    } else {
      let tm = tl + (tr - tl) / 2;
      Self::build(arr, data, v * 2 + 1, tl, tm, merge);
      Self::build(arr, data, v * 2 + 2, tm + 1, tr, merge);
      data[v] = merge(data[v * 2 + 1], data[v * 2 + 2]);
    }
  }

  pub fn update(&mut self, pos: usize, new_val: T) {
    if pos >= self.size {
      return;
    }

    self.update_rec(0, 0, self.size - 1, pos, new_val);
  }

  fn update_rec(&mut self, v: usize, tl: usize, tr: usize, pos: usize, new_val: T) {
    if tl == tr {
      self.data[v] = new_val;
      return;
    }

    let tm = tl + (tr - tl) / 2;

    if pos <= tm {
      self.update_rec(v * 2 + 1, tl, tm, pos, new_val);
    } else {
      self.update_rec(v * 2 + 2, tm + 1, tr, pos, new_val);
    }

    self.data[v] = (self.merge)(self.data[v * 2 + 1], self.data[v * 2 + 2]);
  }

  pub fn find_first_great_or_equal(&self, v: usize, value: T) -> Option<usize>
  where
    T: PartialOrd + PartialEq,
  {
    self.find_first_great_or_equal_rec(v, 0, self.size - 1, value)
  }

  fn find_first_great_or_equal_rec(&self, v: usize, tl: usize, tr: usize, value: T) -> Option<usize>
  where
    T: PartialOrd + PartialEq,
  {
    if self.data[v] < value {
      return None;
    }

    if tl == tr {
      return Some(tl);
    }

    let tm = tl + (tr - tl) / 2;
    let left_child = v * 2 + 1;

    if self.data[left_child] >= value {
      self.find_first_great_or_equal_rec(left_child, tl, tm, value)
    } else {
      let right_child = v * 2 + 2;
      self.find_first_great_or_equal_rec(right_child, tm + 1, tr, value)
    }
  }
}

struct Solution;

impl Solution {
  pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    let size = baskets.len();
    if size == 0 {
      return fruits.len() as i32;
    }

    let mut unplaced = 0;
    let mut tree = SegmentTree::new(&baskets, i32::MIN, |a, b| a.max(b));

    for &fruit in fruits.iter() {
      if let Some(pos) = tree.find_first_great_or_equal(0, fruit) {
        tree.update(pos, i32::MIN);
      } else {
        unplaced += 1;
      }
    }

    unplaced
  }
}

#[test]
fn test() {
  let test_cases = [(vec![4, 2, 5], vec![3, 5, 4], 1), (vec![3, 6, 1], vec![6, 4, 7], 0)];

  for (fruits, baskets, expected) in test_cases {
    assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), expected);
  }
}
