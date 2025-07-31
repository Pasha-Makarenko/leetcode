// 898. Bitwise ORs of Subarrays

use std::collections::HashSet;

struct Solution;

impl Solution {
  pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
    let mut result = HashSet::new();
    let mut current = HashSet::new();

    for &num in &arr {
      let mut next = HashSet::new();
      next.insert(num);

      for &val in &current {
        next.insert(val | num);
      }

      current = next;

      for &val in &current {
        result.insert(val);
      }
    }

    result.len() as i32
  }
}

#[test]
fn test() {
  let test_cases = [(vec![0], 1), (vec![1, 1, 2], 3), (vec![1, 2, 4], 6)];

  for (arr, expected) in test_cases {
    assert_eq!(Solution::subarray_bitwise_o_rs(arr), expected);
  }
}
