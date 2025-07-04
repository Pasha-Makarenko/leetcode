// 3304. Find the K-th Character in String Game I

struct Solution;

impl Solution {
  pub fn kth_character(k: i32) -> char {
    (b'a' + (k - 1).count_ones() as u8).into()
  }
}

#[test]
fn test() {
  let teset_cases = [(5, 'b'), (10, 'c')];

  for (k, expected) in teset_cases {
    assert_eq!(Solution::kth_character(k), expected);
  }
}
