// 3439. Reschedule Meetings for Maximum Free Time I

struct Solution;

impl Solution {
  pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    if k >= event_time {
      return event_time;
    }

    let k = k as usize;
    let meets = start_time.len();
    let mut sum = start_time[0];
    let mut max = 0;

    for i in 1..=meets {
      sum +=
        if i < meets { start_time[i] - end_time[i - 1] } else { event_time - end_time[meets - 1] };

      if i > k + 1 {
        sum -= start_time[i - k - 1] - end_time[i - k - 2];
      } else if i == k + 1 {
        sum -= start_time[0];
      }

      if i >= k && sum > max {
        max = sum;
      }
    }

    max
  }
}

#[test]
fn test() {
  let test_cases = [
    (5, 1, vec![1, 3], vec![2, 5], 2),
    (10, 1, vec![0, 2, 9], vec![1, 4, 10], 6),
    (5, 2, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5], 0),
  ];

  for (event_time, k, start_time, end_time, expected) in test_cases {
    assert_eq!(Solution::max_free_time(event_time, k, start_time, end_time), expected);
  }
}
