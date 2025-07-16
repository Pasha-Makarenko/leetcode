// 3201. Find the Maximum Length of Valid Subsequence I

struct Solution;

impl Solution {
  pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let (mut even, mut odd, mut alt, mut prev) = ((1 - nums[0]) & 1, nums[0] & 1, 1, nums[0] & 1);

    for &num in nums.iter().skip(1) {
      let parity = num & 1;

      if parity == 0 {
        even += 1;
      } else {
        odd += 1;
      }

      if prev != parity {
        alt += 1;
        prev = parity;
      }
    }

    even.max(odd).max(alt)
  }
}

#[test]
fn test() {
  let test_cases = [(vec![1, 2, 3, 4], 4), (vec![1, 2, 1, 1, 2, 1, 2], 6), (vec![1, 3], 2)];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::maximum_length(nums), expected);
  }
}
