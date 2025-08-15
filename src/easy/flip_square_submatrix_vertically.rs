// 3643. Flip Square Submatrix Vertically

struct Solution;

impl Solution {
  pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
    let mut new_grid = grid;
    let x = x as usize;
    let y = y as usize;
    let k = k as usize;

    for i in 0..k / 2 {
      for col in y..y + k {
        let temp = new_grid[x + i][col];
        new_grid[x + i][col] = new_grid[x + k - 1 - i][col];
        new_grid[x + k - 1 - i][col] = temp;
      }
    }

    new_grid
  }
}

#[test]
fn test() {
  let test_cases = [
    (
      vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]],
      1,
      0,
      3,
      vec![vec![1, 2, 3, 4], vec![13, 14, 15, 8], vec![9, 10, 11, 12], vec![5, 6, 7, 16]],
    ),
    (
      vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]],
      0,
      2,
      2,
      vec![vec![3, 4, 4, 2], vec![2, 3, 2, 3]],
    ),
  ];

  for (grid, x, y, k, expected) in test_cases {
    assert_eq!(Solution::reverse_submatrix(grid, x, y, k), expected);
  }
}
