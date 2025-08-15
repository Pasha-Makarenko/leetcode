// 342. Power of Four

struct Solution;

impl Solution {
  pub fn is_power_of_four(n: i32) -> bool {
    n.count_ones() == 1 && n.trailing_zeros() & 1 == 0 && n != i32::MIN
  }
}

#[test]
fn test() {
  let test_cases = [(16, true), (5, false), (1, true)];

  for (n, expected) in test_cases {
    assert_eq!(Solution::is_power_of_four(n), expected);
  }
}
