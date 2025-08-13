// 326. Power of Three

struct Solution;

impl Solution {
  pub fn is_power_of_three(n: i32) -> bool {
    if n <= 0 {
      return false;
    }

    let log = (n as f64).log(3.0);

    (log - log.round()).abs() <= 0.00000000001
  }
}

#[test]
fn test() {
  let test_cases = [(27, true), (0, false), (-1, false)];

  for (n, expected) in test_cases {
    assert_eq!(Solution::is_power_of_three(n), expected);
  }
}
