// 808. Soup Servings

struct Solution;

use std::collections::HashMap;

impl Solution {
  pub fn soup_servings(n: i32) -> f64 {
    if n >= 4800 {
      return 1.0;
    }

    let n = (n + 24) / 25;
    let mut memo = HashMap::new();

    Self::dp(n, n, &mut memo)
  }

  fn dp(a: i32, b: i32, memo: &mut HashMap<(i32, i32), f64>) -> f64 {
    if a <= 0 && b <= 0 {
      return 0.5;
    }
    if a <= 0 {
      return 1.0;
    }
    if b <= 0 {
      return 0.0;
    }

    if let Some(&res) = memo.get(&(a, b)) {
      return res;
    }

    let res = 0.25
      * (Self::dp(a - 4, b, memo)
        + Self::dp(a - 3, b - 1, memo)
        + Self::dp(a - 2, b - 2, memo)
        + Self::dp(a - 1, b - 3, memo));

    memo.insert((a, b), res);
    res
  }
}

#[test]
fn test() {
  let test_cases = [(50, 0.625), (100, 0.71875)];

  for (n, expected) in test_cases {
    assert_eq!(Solution::soup_servings(n), expected);
  }
}
