// 3330. Find the Original Typed String I

struct Solution;

impl Solution {
  pub fn possible_string_count(word: String) -> i32 {
    let mut result = 1;
    let mut prev = None;

    for c in word.chars() {
      if let Some(p) = prev {
        if p == c {
          result += 1;
        }
      }
      prev = Some(c);
    }

    result
  }
}

#[test]
fn test() {
  let test_cases =
    [(String::from("abbcccc"), 5), (String::from("abcd"), 1), (String::from("aaaa"), 4)];

  for (str, expected) in test_cases {
    assert_eq!(Solution::possible_string_count(str), expected);
  }
}
