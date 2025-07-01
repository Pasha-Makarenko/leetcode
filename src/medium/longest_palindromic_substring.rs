// 5. Longest Palindromic Substring

struct Solution;

impl Solution {
  pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
      return String::new();
    }

    let chars: Vec<char> = s.chars().collect();
    let (mut start, mut end) = (0, 0);

    for i in 0..chars.len() {
      let len1 = Solution::expand_around_center(&chars, i, i);
      let len2 = Solution::expand_around_center(&chars, i, i + 1);
      let len = len1.max(len2);

      if len > end - start {
        start = i - (len - 1) / 2;
        end = i + len / 2;
      }
    }

    chars[start..=end].iter().collect()
  }

  fn expand_around_center(chars: &[char], left: usize, right: usize) -> usize {
    let (mut l, mut r) = (left as i32, right as i32);
    while l >= 0 && r < chars.len() as i32 && chars[l as usize] == chars[r as usize] {
      l -= 1;
      r += 1;
    }
    (r - l - 1) as usize
  }
}

#[test]
fn test() {
  let test_cases =
    [(String::from("babad"), String::from("aba")), (String::from("cbbd"), String::from("bb"))];

  for (s, expected) in test_cases {
    assert_eq!(Solution::longest_palindrome(s), expected);
  }
}
