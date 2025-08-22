// 3195. Find the Minimum Area to Cover All Ones I

struct Solution;

impl Solution {
  pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let (mut min_i, mut max_i, mut min_j, mut max_j) = (n, 0, m, 0);

    for i in 0..n {
      for j in 0..m {
        if grid[i][j] == 1 {
          min_i = min_i.min(i);
          max_i = max_i.max(i);
          min_j = min_j.min(j);
          max_j = max_j.max(j);
        }
      }
    }

    ((max_i - min_i + 1) * (max_j - min_j + 1)) as i32
  }
}

#[test]
fn test() {
  let test_cases = [(vec![vec![1, 0, 1], vec![0, 1, 0]], 6), (vec![vec![1, 0], vec![0, 0]], 1)];

  for (grid, expected) in test_cases {
    assert_eq!(Solution::minimum_area(grid), expected);
  }
}
