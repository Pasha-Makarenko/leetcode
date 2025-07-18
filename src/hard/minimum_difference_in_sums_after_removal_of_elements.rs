// 2163. Minimum Difference in Sums After Removal of Elements

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
  pub fn minimum_difference(nums: Vec<i32>) -> i64 {
    let n = nums.len() / 3;
    let mut current_sum = 0i64;

    let mut left_sums = vec![0i64; 2 * n + 1];
    let mut max_heap = BinaryHeap::new();

    for i in 0..(2 * n) {
      current_sum += nums[i] as i64;
      max_heap.push(nums[i]);

      if max_heap.len() > n {
        if let Some(value) = max_heap.pop() {
          current_sum -= value as i64;
        }
      }

      left_sums[i + 1] = current_sum;
    }

    current_sum = 0i64;

    let mut right_sums = vec![0i64; 2 * n + 1];
    let mut min_heap = BinaryHeap::new();

    for i in (n..(3 * n)).rev() {
      current_sum += nums[i] as i64;
      min_heap.push(Reverse(nums[i]));

      if min_heap.len() > n {
        if let Some(Reverse(value)) = min_heap.pop() {
          current_sum -= value as i64;
        }
      }

      right_sums[i - n] = current_sum;
    }

    let mut min_diff = i64::MAX;

    for i in n..=(2 * n) {
      min_diff = min_diff.min(left_sums[i] - right_sums[i - n]);
    }

    min_diff
  }
}

#[test]
fn test() {
  let test_cases = [(vec![3, 1, 2], -1), (vec![7, 9, 5, 8, 1, 3], 1)];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::minimum_difference(nums), expected);
  }
}
