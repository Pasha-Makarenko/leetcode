// 1353. Maximum Number of Events That Can Be Attended

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
  pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
    if events.is_empty() {
      return 0;
    }

    events.sort_by(|a, b| a[0].cmp(&b[0]));

    let events_count = events.len();
    let max_day = events.iter().map(|e| e[1]).max().unwrap();
    let mut result = 0;
    let mut current_event = 0;
    let mut queue = BinaryHeap::new();

    for day in 1..=max_day {
      while current_event < events_count && events[current_event][0] == day {
        queue.push(Reverse(events[current_event][1]));
        current_event += 1;
      }

      while let Some(&Reverse(end)) = queue.peek() {
        if end < day {
          queue.pop();
        } else {
          break;
        }
      }

      if let Some(Reverse(_)) = queue.pop() {
        result += 1;
      }
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [
    (vec![vec![1, 2], vec![2, 3], vec![3, 4]], 3),
    (vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]], 4),
  ];

  for (events, expected) in test_cases {
    assert_eq!(Solution::max_events(events), expected);
  }
}
