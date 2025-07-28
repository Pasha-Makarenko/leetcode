// 2044. Count Number of Maximum Bitwise-OR Subsets

struct Solution;

impl Solution {
  fn dfs(index: usize, current_or: i32, max_or: i32, nums: &Vec<i32>) -> i32 {
    if current_or == max_or {
      return 1 << (nums.len() - index);
    }

    if index == nums.len() {
      return 0;
    }

    Self::dfs(index + 1, current_or | nums[index], max_or, nums)
      + Self::dfs(index + 1, current_or, max_or, nums)
  }

  pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let max_or = nums.iter().fold(0, |acc, &x| acc | x);
    Self::dfs(0, 0, max_or, &nums)
  }
}

#[test]
fn test() {
  let test_cases = [(vec![3, 1], 2), (vec![2, 2, 2], 7), (vec![3, 2, 1, 5], 6)];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::count_max_or_subsets(nums), expected);
  }
}
