// 1865. Finding Pairs With a Certain Sum

use std::collections::HashMap;

struct FindSumPairs {
  nums1: Vec<i32>,
  nums2: Vec<i32>,
  counts: HashMap<i32, i32>,
}

impl FindSumPairs {
  fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
    let mut counts = HashMap::new();

    for &item in nums2.iter() {
      *counts.entry(item).or_insert(0) += 1;
    }

    FindSumPairs { nums1, nums2, counts }
  }

  fn add(&mut self, index: i32, val: i32) {
    if let Some(item) = self.nums2.get_mut(index as usize) {
      *self.counts.entry(*item).or_insert(1) -= 1;

      *item += val;

      *self.counts.entry(*item).or_insert(0) += 1;
    }
  }

  fn count(&self, tot: i32) -> i32 {
    let mut result = 0;

    for &item in self.nums1.iter() {
      if let Some(&count) = self.counts.get(&(tot - item)) {
        result += count;
      }
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [(
    vec!["FindSumPairs", "count", "add", "count", "count", "add", "add", "count"],
    vec![
      vec![vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]],
      vec![vec![7]],
      vec![vec![3], vec![2]],
      vec![vec![8]],
      vec![vec![4]],
      vec![vec![0], vec![1]],
      vec![vec![1], vec![1]],
      vec![vec![7]],
    ],
    vec![None, Some(8), None, Some(2), Some(1), None, None, Some(11)],
  )];

  for (commands, values, expected) in test_cases {
    let mut find_sum_pairs = None;

    for (i, &command) in commands.iter().enumerate() {
      match command {
        "FindSumPairs" => {
          let nums1 = values[i][0].clone();
          let nums2 = values[i][1].clone();
          find_sum_pairs = Some(FindSumPairs::new(nums1, nums2));
        }
        "count" => {
          let tot = values[i][0][0];
          let result = find_sum_pairs.as_ref().unwrap().count(tot);
          assert_eq!(Some(result), expected[i], "Failed at count({tot})");
        }
        "add" => {
          let index = values[i][0][0];
          let val = values[i][1][0];
          find_sum_pairs.as_mut().unwrap().add(index, val);
          assert_eq!(None, expected[i], "Failed at add({index}, {val})");
        }
        _ => panic!("Unknown command: {command}"),
      }
    }
  }
}
