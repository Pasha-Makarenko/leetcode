// 7. Reverse Integer

struct Solution;

impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let sign = x.signum();
    let mut val = x.abs() as i64;
    let mut result = 0;

    while val > 0 {
      let digit = val % 10;
      result = 10 * result + digit;
      val /= 10;
    }

    if result > i32::MAX as i64 {
      return 0;
    }

    sign * result as i32
  }
}

#[test]
fn test() {
  let test_cases = [(123, 321), (-123, -321), (120, 21)];

  for (num, expected) in test_cases {
    assert_eq!(Solution::reverse(num), expected);
  }
}
