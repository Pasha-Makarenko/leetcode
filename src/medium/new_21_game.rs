// 837. New 21 Game

struct Solution;

impl Solution {
  pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    let n = n as usize;
    let k = k as usize;
    let max_pts = max_pts as usize;

    if k == 0 || n >= k + max_pts {
      return 1f64;
    }

    let mut dp = vec![0f64; n + 1];
    dp[0] = 1f64;
    let mut window = 1f64;
    let mut res = 0f64;

    for i in 1..=n {
      dp[i] = window / max_pts as f64;

      if i < k {
        window += dp[i];
      } else {
        res += dp[i];
      }

      if i >= max_pts {
        window -= dp[i - max_pts];
      }
    }

    res
  }
}

#[test]
fn test() {
  let test_cases = [(10, 1, 10, 1.0), (6, 1, 10, 0.6), (21, 17, 10, 0.73278)];

  for (n, k, max_pts, expected) in test_cases {
    assert!((Solution::new21_game(n, k, max_pts) - expected).abs() < 0.00001);
  }
}
