// 1957. Delete Characters to Make Fancy String

struct Solution;

impl Solution {
  pub fn make_fancy_string(s: String) -> String {
    let mut prev = s.chars().next().unwrap();
    let mut result = String::from(prev);
    let mut count = 0;

    for c in s.chars().skip(1) {
      if c == prev {
        count += 1;

        if count >= 2 {
          continue;
        }
      } else {
        count = 0;
        prev = c;
      }

      result.push(c);
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [
    (String::from("leeetcode"), String::from("leetcode")),
    (String::from("aaabaaaa"), String::from("aabaa")),
    (String::from("aab"), String::from("aab")),
  ];

  for (s, expected) in test_cases {
    assert_eq!(Solution::make_fancy_string(s), expected);
  }
}
