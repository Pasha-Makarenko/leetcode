// 3440. Reschedule Meetings for Maximum Free Time II

struct Solution;

impl Solution {
  pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    let len = start_time.len();
    let mut gaps = Vec::<i32>::with_capacity(len + 1);

    gaps.push(start_time[0]);
    for i in 1..len {
      gaps.push(start_time[i] - end_time[i - 1]);
    }
    gaps.push(event_time - end_time[len - 1]);

    let mut max_gap = *gaps.iter().max().unwrap();

    let mut prefix_max = vec![0; len + 1];
    let mut suffix_max = vec![0; len + 1];

    prefix_max[0] = gaps[0];
    for i in 1..=len {
      prefix_max[i] = prefix_max[i - 1].max(gaps[i]);
    }

    suffix_max[len] = gaps[len];
    for i in (0..len).rev() {
      suffix_max[i] = suffix_max[i + 1].max(gaps[i]);
    }

    for i in 0..len {
      let duration = end_time[i] - start_time[i];
      let neighbours_gap = gaps[i] + gaps[i + 1];
      let mut best_gap = neighbours_gap;

      let max_other_gap = if i == 0 {
        suffix_max[i + 2]
      } else if i == len - 1 {
        prefix_max[i - 1]
      } else {
        prefix_max[i - 1].max(suffix_max[i + 2])
      };

      if max_other_gap >= duration {
        best_gap = best_gap.max(neighbours_gap + duration);
      }

      max_gap = max_gap.max(best_gap);
    }

    max_gap
  }
}

#[test]
fn test() {
  let test_cases = [
    (5, vec![1, 3], vec![2, 5], 2),
    (10, vec![0, 7, 9], vec![1, 8, 10], 7),
    (10, vec![0, 3, 7, 9], vec![1, 4, 8, 10], 6),
    (5, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5], 0),
  ];

  for (event_time, start_time, end_time, expected) in test_cases {
    assert_eq!(Solution::max_free_time(event_time, start_time, end_time), expected);
  }
}
