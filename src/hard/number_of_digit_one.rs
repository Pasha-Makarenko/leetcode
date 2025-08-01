// 233. Number of Digit One

struct Solution;

impl Solution {
  pub fn count_digit_one(n: i32) -> i32 {
    let mut count = 0;
    let n = n as i64;
    let mut i = 1;

    while i <= n {
      let divider = i * 10;
      count += (n / divider) * i + (n % divider - i + 1).clamp(0, i);
      i *= 10;
    }

    count as i32
  }
}

#[test]
fn test() {
  let test_cases = [(13, 6), (0, 0)];

  for (n, expected) in test_cases {
    assert_eq!(Solution::count_digit_one(n), expected);
  }
}
