// 2787. Ways to Express an Integer as Sum of Powers

const MODULO: i64 = 1_000_000_007;

struct Solution;

impl Solution {
  fn pow(base: i32, exp: i32) -> i32 {
    let mut res = 1;

    for _ in 0..exp {
      res *= base;
    }

    res
  }

  pub fn number_of_ways(n: i32, x: i32) -> i32 {
    let mut powers = vec![];
    let mut i = 1;

    loop {
      let p = Self::pow(i, x);
      if p > n {
        break;
      }
      powers.push(p);
      i += 1;
    }

    let mut dp = vec![0i64; (n + 1) as usize];
    dp[0] = 1;

    for p in powers {
      for s in (p..=n).rev() {
        dp[s as usize] = (dp[s as usize] + dp[(s - p) as usize]) % MODULO;
      }
    }

    dp[n as usize] as i32
  }
}

#[test]
fn test() {
  let test_cases = [(10, 2, 1), (4, 1, 2)];

  for (n, x, expected) in test_cases {
    assert_eq!(Solution::number_of_ways(n, x), expected);
  }
}
