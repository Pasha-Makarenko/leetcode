// 52. N-Queens II

const EMPTY: char = '.';
const QUEEN: char = 'Q';

struct Solution;

impl Solution {
  fn solve(
    row: i32,
    n: i32,
    cols: &mut Vec<i32>,
    diag1: &mut Vec<i32>,
    diag2: &mut Vec<i32>,
    board: &mut Vec<Vec<char>>,
    result: &mut Vec<Vec<String>>,
  ) {
    if row == n {
      result.push(board.iter().map(|row| row.iter().collect::<String>()).collect());
      return;
    }
    for col in 0..n {
      let d1 = row + col;
      let d2 = row - col;

      if !cols.contains(&col) && !diag1.contains(&d1) && !diag2.contains(&d2) {
        board[row as usize][col as usize] = QUEEN;
        cols.push(col);
        diag1.push(d1);
        diag2.push(d2);

        Self::solve(row + 1, n, cols, diag1, diag2, board, result);

        board[row as usize][col as usize] = EMPTY;
        cols.pop();
        diag1.pop();
        diag2.pop();
      }
    }
  }

  pub fn total_n_queens(n: i32) -> i32 {
    let mut result = Vec::new();
    let mut board = vec![vec![EMPTY; n as usize]; n as usize];
    let mut cols = Vec::new();
    let mut diag1 = Vec::new();
    let mut diag2 = Vec::new();

    Self::solve(0, n, &mut cols, &mut diag1, &mut diag2, &mut board, &mut result);

    result.len() as i32
  }
}

#[test]
fn test() {
  let test_cases = [(4, 2), (1, 1)];

  for (n, expected) in test_cases {
    assert_eq!(Solution::total_n_queens(n), expected);
  }
}
