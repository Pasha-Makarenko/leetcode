// 869. Reordered Power of 2

use std::collections::BTreeMap;

struct Solution;

impl Solution {
  fn counts(mut n: i32) -> BTreeMap<i32, usize> {
    let mut tree = BTreeMap::new();

    while n >= 1 {
      *tree.entry(n % 10).or_insert(0) += 1;
      n /= 10;
    }

    tree
  }

  pub fn reordered_power_of2(n: i32) -> bool {
    let counts = Self::counts(n);
    let mut power = 1;

    if counts == Self::counts(power) {
      return true;
    }

    for _ in 1..31 {
      power <<= 1;

      if counts == Self::counts(power) {
        return true;
      }
    }

    false
  }
}

#[test]
fn test() {
  let test_cases = [(1, true), (10, false)];

  for (n, expected) in test_cases {
    assert_eq!(Solution::reordered_power_of2(n), expected);
  }
}
