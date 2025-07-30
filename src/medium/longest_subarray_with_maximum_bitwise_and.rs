// 2419. Longest Subarray With Maximum Bitwise AND

struct Solution;

impl Solution {
  pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let max_num = *nums.iter().max().unwrap();
    let mut max_len = 0;
    let mut current_len = 0;

    for &num in &nums {
      if num == max_num {
        current_len += 1;
        max_len = max_len.max(current_len);
      } else {
        current_len = 0;
      }
    }

    max_len
  }
}

#[test]
fn test() {
  let test_cases = [(vec![1, 2, 3, 3, 2, 2], 2), (vec![1, 2, 3, 4], 1)];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::longest_subarray(nums), expected);
  }
}
