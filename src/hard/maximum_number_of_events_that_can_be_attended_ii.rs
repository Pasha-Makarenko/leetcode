// 1751. Maximum Number of Events That Can Be Attended II

struct Solution;

impl Solution {
  pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
    if events.is_empty() {
      return 0;
    }

    events.sort_by(|a, b| a[1].cmp(&b[1]));

    let events_count = events.len();
    let mut dp = vec![vec![0; k as usize + 1]; events_count + 1];

    for i in 1..=events_count {
      let (start, _, value) = (events[i - 1][0], events[i - 1][1], events[i - 1][2]);
      let mut left = 0;
      let mut right = i as i32 - 1;
      let mut last_non_overlapping = events_count;

      while left <= right {
        let mid = (left + right) / 2;

        if events[mid as usize][1] < start {
          last_non_overlapping = mid as usize;
          left = mid + 1;
        } else {
          right = mid - 1;
        }
      }

      for j in 1..=(k as usize) {
        if last_non_overlapping != events_count {
          dp[i][j] = dp[i - 1][j].max(value + dp[last_non_overlapping + 1][j - 1]);
        } else {
          dp[i][j] = dp[i - 1][j].max(value);
        }
      }
    }

    dp[events_count].iter().max().cloned().unwrap_or(0)
  }
}

#[test]
fn test() {
  let test_cases = [
    (vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2, 7),
    (vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]], 2, 10),
    (vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]], 3, 9),
  ];

  for (events, k, expected) in test_cases {
    assert_eq!(Solution::max_value(events, k), expected);
  }
}
