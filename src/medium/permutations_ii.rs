// 47. Permutations II

use std::collections::HashSet;

struct Solution;

impl Solution {
  fn permutations(index: usize, nums: &mut Vec<i32>, perms: &mut Vec<Vec<i32>>) {
    if index == nums.len() {
      perms.push(nums.clone());
      return;
    }

    let mut swape = HashSet::new();

    for digit in index..nums.len() {
      if swape.contains(&nums[digit]) {
        continue;
      }

      swape.insert(nums[digit]);
      nums.swap(digit, index);
      Self::permutations(index + 1, nums, perms);
      nums.swap(digit, index);
    }
  }

  pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut perms = Vec::new();
    Self::permutations(0, &mut nums, &mut perms);
    perms
  }
}

#[test]
fn test() {
  let test_cases = [
    (vec![1, 1, 2], vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]),
    (
      vec![1, 2, 3],
      vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 2, 1],
        vec![3, 1, 2],
      ],
    ),
  ];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::permute_unique(nums), expected);
  }
}
