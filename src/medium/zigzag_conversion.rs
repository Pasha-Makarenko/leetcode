// 6. Zigzag Conversion

struct Solution;

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
      return s;
    }

    let length = s.len() as i32;
    let period = 2 * (num_rows - 1);
    let mut result = String::with_capacity(length as usize);

    for row in 0..num_rows {
      let mut i = 0;

      while period * i < length {
        let hor = period * i + row;

        if hor < length {
          result.push(s.chars().nth(hor as usize).unwrap());
        }

        if row != 0 && row != num_rows - 1 {
          let diag = period * (i + 1) - row;

          if diag < length {
            result.push(s.chars().nth(diag as usize).unwrap());
          }
        }

        i += 1;
      }
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [
    (String::from("PAYPALISHIRING"), 3, String::from("PAHNAPLSIIGYIR")),
    (String::from("PAYPALISHIRING"), 4, String::from("PINALSIGYAHRPI")),
  ];

  for (str, num_rows, expected) in test_cases {
    assert_eq!(Solution::convert(str, num_rows), expected);
  }
}
