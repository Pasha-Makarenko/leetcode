// 3000. Maximum Area of Longest Diagonal Rectangle

struct Solution;

impl Solution {
  pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
    let mut rect = (0, 0);
    let mut max = 0;

    for r in dimensions.iter() {
      let diag = r[0] * r[0] + r[1] * r[1];

      if diag > max {
        max = diag;
        rect = (r[0], r[1]);
      } else if diag == max && rect.0 * rect.1 < r[0] * r[1] {
        rect = (r[0], r[1]);
      }
    }

    rect.0 * rect.1
  }
}

#[test]
fn test() {
  let test_cases = [(vec![vec![9, 3], vec![8, 6]], 48), (vec![vec![3, 4], vec![4, 3]], 12)];

  for (dimensions, expected) in test_cases {
    assert_eq!(Solution::area_of_max_diagonal(dimensions), expected);
  }
}
