// 10. Regular Expression Matching

const ANY_SYMBOL: char = '.';
const REPEATS: char = '*';

struct Solution;

impl Solution {
  pub fn dp(i: usize, j: usize, s: &[char], p: &[char], memo: &mut Vec<Vec<Option<bool>>>) -> bool {
    if let Some(&cached) = memo[i][j].as_ref() {
      return cached;
    }

    let res = if j == p.len() {
      i == s.len()
    } else {
      let first_match = i < s.len() && (p[j] == ANY_SYMBOL || p[j] == s[i]);

      if j + 1 < p.len() && p[j + 1] == REPEATS {
        Solution::dp(i, j + 2, s, p, memo) || (first_match && Solution::dp(i + 1, j, s, p, memo))
      } else {
        first_match && Solution::dp(i + 1, j + 1, s, p, memo)
      }
    };

    memo[i][j] = Some(res);
    res
  }

  pub fn is_match(s: String, p: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();

    let mut memo = vec![vec![None; p_chars.len() + 1]; s_chars.len() + 1];

    Solution::dp(0, 0, &s_chars, &p_chars, &mut memo)
  }
}

#[test]
fn test() {
  let test_cases = [
    (String::from("aa"), String::from("a"), false),
    (String::from("aa"), String::from("a*"), true),
    (String::from("ab"), String::from(".*"), true),
  ];

  for (s, p, expected) in test_cases {
    assert_eq!(Solution::is_match(s, p), expected);
  }
}
