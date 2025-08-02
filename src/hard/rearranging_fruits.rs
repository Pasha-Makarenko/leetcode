// 2561. Rearranging Fruits

use std::collections::HashMap;

struct Solution;

impl Solution {
  pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
    let n = basket1.len();
    let mut freq = HashMap::new();
    let mut min_fruit = i32::MAX;

    for i in 0..n {
      *freq.entry(basket1[i]).or_insert(0i32) += 1;
      *freq.entry(basket2[i]).or_insert(0i32) -= 1;
      min_fruit = min_fruit.min(basket1[i]).min(basket2[i]);
    }

    for &count in freq.values() {
      if count & 1 == 1 {
        return -1;
      }
    }

    let mut swap_from_b1 = Vec::new();
    let mut swap_from_b2 = Vec::new();

    for (&fruit, &count) in freq.iter() {
      let diff = count.abs() / 2;

      if diff == 0 {
        continue;
      }

      let fruit_list = vec![fruit; diff as usize];

      if count > 0 {
        swap_from_b1.extend(fruit_list);
      } else {
        swap_from_b2.extend(fruit_list);
      }
    }

    swap_from_b1.sort_unstable();
    swap_from_b2.sort_unstable_by(|a, b| b.cmp(a));

    let mut total_cost = 0i64;

    for i in 0..swap_from_b1.len() {
      let cost = swap_from_b1[i].min(swap_from_b2[i]);
      total_cost += cost.min(2 * min_fruit) as i64;
    }

    total_cost
  }
}

#[test]
fn test() {
  let test_cases =
    [(vec![4, 2, 2, 2], vec![1, 4, 1, 2], 1), (vec![2, 3, 4, 1], vec![3, 2, 5, 1], -1)];

  for (basket1, basket2, expected) in test_cases {
    assert_eq!(Solution::min_cost(basket1, basket2), expected);
  }
}
