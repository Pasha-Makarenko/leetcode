// 2411. Smallest Subarrays With Maximum Bitwise OR

struct Solution;

impl Solution {
  pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
    let length = nums.len();
    let mut max_or = vec![0i32; length];
    max_or[length - 1] = nums[length - 1];

    for i in (0..length - 1).rev() {
      max_or[i] = max_or[i + 1] | nums[i];
    }

    let mut counts = vec![1i32; length];
    let mut last_bit_pos = [-1; 32];

    for i in (0..length).rev() {
      for (bit, pos) in last_bit_pos.iter_mut().enumerate() {
        if (nums[i] >> bit) & 1 != 0 {
          *pos = i as i32;
        }
      }

      let mut max_pos = i as i32;
      for (bit, pos) in last_bit_pos.iter().enumerate() {
        if (max_or[i] >> bit) & 1 != 0 {
          max_pos = max_pos.max(*pos);
        }
      }

      counts[i] = max_pos - i as i32 + 1;
    }

    counts
  }
}

#[test]
fn test() {
  let test_cases = [(vec![1, 0, 2, 1, 3], vec![3, 3, 2, 2, 1]), (vec![1, 2], vec![2, 1])];

  for (nums, expected) in test_cases {
    assert_eq!(Solution::smallest_subarrays(nums), expected);
  }
}
