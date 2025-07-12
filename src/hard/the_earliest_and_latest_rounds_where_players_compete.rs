// 1900. The Earliest and Latest Rounds Where Players Compete

use std::collections::HashMap;

struct Solution;

impl Solution {
  fn dfs(count: i32, p1: i32, p2: i32, memo: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let (mut p1, mut p2) = (p1.min(p2), p1.max(p2));

    if let Some(cached_result) = memo.get(&((count << 10) | (p1 << 5) | p2)) {
      return cached_result.clone();
    }

    if p1 + p2 == count + 1 {
      return vec![1, 1];
    }

    if count <= 4 {
      return vec![2, 2];
    }

    let next_count = (count + 1) / 2;
    let (mut min_round, mut max_round) = (i32::MAX, i32::MIN);

    if p1 + p2 > count + 1 {
      let new_p2 = count + 1 - p1;
      p1 = count + 1 - p2;
      p2 = new_p2;
    }

    let left = p1 - 1;
    let beetwen = if 2 * p2 <= count + 1 { p2 - p1 - 1 } else { count - p2 - p1 };
    let offset = if 2 * p2 <= count + 1 { 0 } else { (2 * p2 - count - 1) / 2 };

    for i in 0..=left {
      for j in 0..=beetwen {
        let result = Solution::dfs(next_count, i + 1, i + j + 2 + offset, memo);
        min_round = min_round.min(result[0] + 1);
        max_round = max_round.max(result[1] + 1);
      }
    }

    memo.insert((count << 10) | (p1 << 5) | p2, vec![min_round, max_round]);
    vec![min_round, max_round]
  }

  pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
    let mut memo = HashMap::new();
    Solution::dfs(n, first_player, second_player, &mut memo)
  }
}

#[test]
fn test() {
  let test_cases = [(11, 2, 4, vec![3, 4]), (5, 1, 5, vec![1, 1])];

  for (n, first_player, second_player, expected) in test_cases {
    assert_eq!(Solution::earliest_and_latest(n, first_player, second_player), expected);
  }
}
