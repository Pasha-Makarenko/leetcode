// 118. Pascal's Triangle

struct Solution;

impl Solution {
  pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let num_rows = num_rows as usize;
    let mut rows = Vec::with_capacity(num_rows);
    rows.push(vec![1]);

    for i in 1..num_rows {
      let mut row = Vec::with_capacity(i + 1);
      row.push(1);

      for j in 1..i {
        row.push(rows[i - 1][j - 1] + rows[i - 1][j]);
      }

      row.push(1);
      rows.push(row);
    }

    rows
  }
}

#[test]
fn test() {
  let test_cases = [
    (
      5,
      vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]],
    ),
    (1, vec![vec![1]]),
  ];

  for (num_rows, expected) in test_cases {
    assert_eq!(Solution::generate(num_rows), expected);
  }
}
