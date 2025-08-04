// 904. Fruit Into Baskets

use std::collections::HashMap;

struct Solution;

impl Solution {
  pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    let mut left = 0;
    let mut result = 0;

    for (right, id) in fruits.iter().enumerate() {
      counts.entry(id).and_modify(|count| *count += 1).or_insert(1);

      while counts.len() > 2 {
        let left_id = fruits[left];

        if let Some(count) = counts.get_mut(&left_id) {
          *count -= 1;
          if *count == 0 {
            counts.remove(&left_id);
          }
        }

        left += 1;
      }

      result = result.max((right - left + 1) as i32);
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [(vec![1, 2, 1], 3), (vec![0, 1, 2, 2], 3), (vec![1, 2, 3, 2, 2], 4)];

  for (fruits, expected) in test_cases {
    assert_eq!(Solution::total_fruit(fruits), expected);
  }
}
