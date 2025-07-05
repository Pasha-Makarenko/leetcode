// 1394. Find Lucky Integer in an Array

use std::collections::HashMap;

struct Solution;

impl Solution {
  pub fn find_lucky(arr: Vec<i32>) -> i32 {
    let mut target = -1;
    let mut freq = HashMap::new();

    for &item in &arr {
      *freq.entry(item).or_insert(0) += 1;
    }

    for (&item, &count) in &freq {
      if item == count && item > target {
        target = count;
      }
    }

    target
  }
}

#[test]
fn test() {
  let test_cases = [(vec![2, 2, 3, 4], 2), (vec![1, 2, 2, 3, 3, 3], 3), (vec![2, 2, 2, 3, 3], -1)];

  for (arr, expected) in test_cases {
    assert_eq!(Solution::find_lucky(arr), expected);
  }
}
