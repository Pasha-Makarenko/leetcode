// 1277. Count Square Submatrices with All Ones

struct Solution;

impl Solution {
  pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut result = 0;

    for i in 0..n {
      for j in 0..m {
        if matrix[i][j] == 0 {
          continue;
        }

        result += 1;
        let mut layer = 1_usize;

        while layer + i < n && layer + j < m {
          let mut valid = true;

          for k in i..=i + layer {
            if matrix[k][j + layer] == 0 {
              valid = false;
              break;
            }
          }

          if valid {
            for k in j..=j + layer {
              if matrix[i + layer][k] == 0 {
                valid = false;
                break;
              }
            }
          }

          if !valid {
            break;
          }

          result += 1;
          layer += 1;
        }
      }
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [
    (vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]], 15),
    (vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]], 7),
  ];

  for (matrix, expected) in test_cases {
    assert_eq!(Solution::count_squares(matrix), expected);
  }
}
