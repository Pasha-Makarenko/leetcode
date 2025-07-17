// 3202. Find the Maximum Length of Valid Subsequence II

struct Solution;

impl Solution {
  pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let nums_len = nums.len();

    if k == 1 {
      return nums_len as i32;
    }

    let mut result = 2;
    let reminders = nums.iter().map(|&n| n % k).collect::<Vec<i32>>();

    for current in 0..k {
      let mut dp = vec![0; k as usize];

      for &reminder in reminders.iter().take(nums_len) {
        let wanted = (current - reminder + k) % k;
        dp[reminder as usize] = dp[wanted as usize] + 1;
        result = result.max(dp[reminder as usize]);
      }
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [(vec![1, 2, 3, 4, 5], 2, 5), (vec![1, 4, 2, 3, 1, 4], 3, 4)];

  for (nums, k, expected) in test_cases {
    assert_eq!(Solution::maximum_length(nums, k), expected);
  }
}
