// 37. Sudoku Solver

const EMPTY: char = '.';
const SIZE: usize = 9;
const SUBSQUARE: usize = 3;
const START_CHAR: char = '1';
const END_CHAR: char = '9';

struct Solution;

impl Solution {
  fn solve(
    pos: usize,
    rows: &mut Vec<Vec<char>>,
    columns: &mut Vec<Vec<char>>,
    subsquares: &mut Vec<Vec<char>>,
    board: &mut Vec<Vec<char>>,
  ) -> bool {
    if pos == SIZE * SIZE {
      return true;
    }

    let row = pos / SIZE;
    let column = pos % SIZE;
    let subsquare = SUBSQUARE * (row / SUBSQUARE) + (column / SUBSQUARE);

    if board[row][column] != EMPTY {
      return Self::solve(pos + 1, rows, columns, subsquares, board);
    }

    for num in START_CHAR..=END_CHAR {
      if rows[row].contains(&num)
        || columns[column].contains(&num)
        || subsquares[subsquare].contains(&num)
      {
        continue;
      }

      board[row][column] = num;
      rows[row].push(num);
      columns[column].push(num);
      subsquares[subsquare].push(num);

      if Self::solve(pos, rows, columns, subsquares, board) {
        return true;
      }

      board[row][column] = EMPTY;
      rows[row].retain(|&x| x != num);
      columns[column].retain(|&x| x != num);
      subsquares[subsquare].retain(|&x| x != num);
    }

    false
  }

  pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut rows = vec![vec![]; SIZE];
    let mut columns = vec![vec![]; SIZE];
    let mut subsquares = vec![vec![]; SIZE];

    for row in 0..SIZE {
      #[allow(clippy::needless_range_loop)]
      for column in 0..SIZE {
        let num = board[row][column];

        if num != EMPTY {
          let subsquare = SUBSQUARE * (row / SUBSQUARE) + (column / SUBSQUARE);

          rows[row].push(num);
          columns[column].push(num);
          subsquares[subsquare].push(num);
        }
      }
    }

    Self::solve(0, &mut rows, &mut columns, &mut subsquares, board);
  }
}

#[test]
fn test() {
  let test_cases = [(
    vec![
      vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
      vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ],
    vec![
      vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
      vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
      vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
      vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
      vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
      vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
      vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
      vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
      vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
    ],
  )];

  for (mut board, expected) in test_cases {
    Solution::solve_sudoku(&mut board);

    assert_eq!(board, expected);
  }
}
