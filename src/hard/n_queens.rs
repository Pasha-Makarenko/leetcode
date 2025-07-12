// 51. N-Queens

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

  pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut board = vec![vec![EMPTY; n as usize]; n as usize];
    let mut cols = Vec::new();
    let mut diag1 = Vec::new();
    let mut diag2 = Vec::new();

    Self::solve(0, n, &mut cols, &mut diag1, &mut diag2, &mut board, &mut result);

    result
  }
}

#[test]
fn test() {
  let test_cases = [
    (
      4,
      vec![
        vec![
          String::from(".Q.."),
          String::from("...Q"),
          String::from("Q..."),
          String::from("..Q."),
        ],
        vec![
          String::from("..Q."),
          String::from("Q..."),
          String::from("...Q"),
          String::from(".Q.."),
        ],
      ],
    ),
    (1, vec![vec![String::from("Q")]]),
  ];

  for (n, expected) in test_cases {
    assert_eq!(Solution::solve_n_queens(n), expected);
  }
}
