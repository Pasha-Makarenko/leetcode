// 3. Longest Substring Without Repeating Characters

struct Solution;

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut result = 0;
    let mut temp = 0;
    let mut start = 0;
    let mut freq = std::collections::HashMap::new();

    for (i, c) in s.chars().enumerate() {
      if let Some(&pos) = freq.get(&c) {
        if pos >= start {
          result = result.max(temp);
          start = pos + 1;
          temp = i - start;
        }
      }

      freq.insert(c, i);
      temp += 1;
    }

    result.max(temp) as i32
  }
}

#[test]
fn test() {
  let test_cases =
    [(String::from("abcabcbb"), 3), (String::from("bbbb"), 1), (String::from("pwwkew"), 3)];

  for (str, expected) in test_cases {
    assert_eq!(Solution::length_of_longest_substring(str), expected);
  }
}
