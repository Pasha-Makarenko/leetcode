// 1695. Maximum Erasure Value

use std::collections::HashSet;

struct Solution;

impl Solution {
  pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::with_capacity(nums.len());
    let mut result = 0;
    let mut temp = 0;
    let mut left = 0;

    for &num in &nums {
      while set.contains(&num) {
        set.remove(&nums[left]);
        temp -= nums[left];
        left += 1;
      }

      set.insert(num);
      temp += num;
      result = result.max(temp);
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [(vec![4, 2, 4, 5, 6], 17), (vec![5, 2, 1, 2, 5, 2, 1, 2, 5], 8)];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::maximum_unique_subarray(nums), expected);
  }
}
