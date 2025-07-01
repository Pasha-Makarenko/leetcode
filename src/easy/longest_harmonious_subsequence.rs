// 594. Longest Harmonious Subsequence

struct Solution;

impl Solution {
  pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut max_length: i32 = 0;
    let mut freq = std::collections::HashMap::new();

    for &num in &nums {
      *freq.entry(num).or_insert(0) += 1;
    }

    for (&num, &count) in &freq {
      if let Some(&next_count) = freq.get(&(num + 1)) {
        max_length = max_length.max(count + next_count);
      }
    }

    max_length
  }
}

#[test]
fn test() {
  let test_cases =
    [(vec![1, 3, 2, 2, 5, 2, 3, 7], 5), (vec![1, 2, 3, 4], 2), (vec![1, 1, 1, 1], 0)];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::find_lhs(nums), expected);
  }
}
