// 3307. Find the K-th Character in String Game II

struct Solution;

impl Solution {
  pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
    let mut k = (k - 1) as i128;
    let mut shift = 0;
    let mut current_length = 1i128 << operations.len();

    for &op in operations.iter().rev() {
      current_length /= 2;
      if k >= current_length {
        if op == 1 {
          shift += 1;
        }
        k -= current_length;
      }
    }

    (b'a' + (shift % 26) as u8) as char
  }
}

#[test]
fn test() {
  let test_cases = [(5, vec![0, 0, 0], 'a'), (10, vec![0, 1, 0, 1], 'b')];

  for (k, operations, expected) in test_cases {
    assert_eq!(Solution::kth_character(k, operations), expected);
  }
}
