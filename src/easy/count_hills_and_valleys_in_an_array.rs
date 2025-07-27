// 2210. Count Hills and Valleys in an Array

struct Solution;

impl Solution {
  pub fn count_hill_valley(mut nums: Vec<i32>) -> i32 {
    nums.dedup();
    nums.windows(3).filter(|w| w[0] < w[1] && w[2] < w[1] || w[0] > w[1] && w[2] > w[1]).count()
      as i32
  }
}

#[test]
fn test() {
  let test_cases = [(vec![2, 4, 1, 1, 6, 5], 3), (vec![6, 6, 5, 5, 4, 1], 0)];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::count_hill_valley(nums), expected);
  }
}
