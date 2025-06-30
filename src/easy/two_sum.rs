// 1. Two Sum

struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = std::collections::HashMap::new();

    for (i, num) in nums.iter().enumerate() {
      if let Some(&index) = hash.get(&(target - num)) {
        return vec![index, i as i32];
      }
      hash.insert(num, i as i32);
    }
    panic!("No solution found");
  }
}

#[test]
fn test() {
  let test_cases = [
    (vec![2, 7, 11, 15], 9, vec![0, 1]),
    (vec![3, 2, 4], 6, vec![1, 2]),
    (vec![3, 3], 6, vec![0, 1]),
  ];

  for (nums, target, expected) in test_cases {
    assert_eq!(Solution::two_sum(nums, target), expected);
  }
}
