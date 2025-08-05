// 3477. Fruits Into Baskets II

struct Solution;

impl Solution {
  pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    let mut unplaced = 0;
    let mut filled = vec![false; baskets.len()];

    for fruit in fruits.iter() {
      let mut is_placed = false;

      for (i, basket) in baskets.iter().enumerate() {
        if !filled[i] && fruit <= basket {
          filled[i] = true;
          is_placed = true;
          break;
        }
      }

      if !is_placed {
        unplaced += 1;
      }
    }

    unplaced
  }
}

#[test]
fn test() {
  let test_cases = [(vec![4, 2, 5], vec![3, 5, 4], 1), (vec![3, 6, 1], vec![6, 4, 7], 0)];

  for (fruits, baskets, expected) in test_cases {
    assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), expected);
  }
}
