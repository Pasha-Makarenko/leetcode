// 3487. Maximum Unique Subarray Sum After Deletion

use std::collections::HashSet;

struct Solution;

impl Solution {
  pub fn max_sum(nums: Vec<i32>) -> i32 {
    let mut unique_positives = HashSet::new();
    let mut max_negative = i32::MIN;
    let mut sum = 0;

    for &num in &nums {
      if num > 0 {
        if unique_positives.insert(num) {
          sum += num;
        }
      } else if num > max_negative {
        max_negative = num;
      }
    }

    if sum > 0 {
      sum
    } else {
      max_negative
    }
  }
}

#[test]
fn test() {
  let test_cases =
    [(vec![1, 2, 3, 4, 5], 15), (vec![1, 1, 0, 1, 1], 1), (vec![1, 2, -1, -2, 1, 0, -1], 3)];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::max_sum(nums), expected);
  }
}
