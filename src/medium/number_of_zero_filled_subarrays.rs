// 2348. Number of Zero-Filled Subarrays

struct Solution;

impl Solution {
  pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    let mut result = 0;
    let mut current = 0;

    for &num in &nums {
      if num == 0 {
        current += 1;
        result += current;
      } else {
        current = 0;
      }
    }

    result
  }
}

#[test]
fn test() {
  let test_cases =
    [(vec![1, 3, 0, 0, 2, 0, 0, 4], 6), (vec![0, 0, 0, 2, 0, 0], 9), (vec![2, 10, 2019], 0)];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::zero_filled_subarray(nums), expected);
  }
}
