// 231. Power of Two

struct Solution;

impl Solution {
  pub fn is_power_of_two(n: i32) -> bool {
    n.count_ones() == 1 && n != i32::MIN
  }
}

#[test]
fn test() {
  let test_cases = [(1, true), (16, true), (3, false)];

  for (n, expected) in test_cases {
    assert_eq!(Solution::is_power_of_two(n), expected);
  }
}
