// 3480. Maximize Subarrays After Removing One Conflicting Pair

struct Solution;

impl Solution {
  pub fn max_subarrays(n: i32, mut conflicting_pairs: Vec<Vec<i32>>) -> i64 {
    let n = n as i64;

    for pair in conflicting_pairs.iter_mut() {
      if pair[0] > pair[1] {
        pair.swap(0, 1);
      }
    }
    conflicting_pairs.sort_by_key(|pair| pair[1]);

    let total_subarrays = n * (n + 1) / 2;
    let mut total_conflicts = 0;
    let mut max_gain = 0;
    let mut max_first = 0;
    let mut max_second = 0;
    let mut gain = 0;

    for (i, pair) in conflicting_pairs.iter().enumerate() {
      let left = pair[0] as i64;

      let base = if i < conflicting_pairs.len() - 1 {
        (conflicting_pairs[i + 1][1] - pair[1]) as i64
      } else {
        n + 1 - pair[1] as i64
      };

      if left > max_first {
        max_second = max_first;
        max_first = left;
        gain = 0;
      } else if left > max_second {
        max_second = left;
      }

      gain += (max_first - max_second) * base;
      total_conflicts += max_first * base;
      max_gain = max_gain.max(gain);
    }

    total_subarrays - total_conflicts + max_gain
  }
}

#[test]
fn test() {
  let test_cases =
    [(4, vec![vec![2, 3], vec![1, 4]], 9), (5, vec![vec![1, 2], vec![2, 5], vec![3, 5]], 12)];

  for (n, conflicting_pairs, expected) in test_cases {
    assert_eq!(Solution::max_subarrays(n, conflicting_pairs), expected);
  }
}
