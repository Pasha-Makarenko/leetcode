// 3136. Valid Word

struct Solution;

impl Solution {
  pub fn is_valid(word: String) -> bool {
    if word.len() < 3 {
      return false;
    }

    let (mut has_vowel, mut has_consonant) = (false, false);

    for c in word.chars() {
      if !(c.is_ascii_digit() || c.is_ascii_alphabetic()) {
        return false;
      }

      if c.is_ascii_alphabetic() {
        let lower_c = c.to_ascii_lowercase();
        match lower_c {
          'a' | 'e' | 'i' | 'o' | 'u' => has_vowel = true,
          _ => has_consonant = true,
        }
      }
    }

    has_vowel && has_consonant
  }
}

#[test]
fn test() {
  let test_cases =
    [(String::from("234Adas"), true), (String::from("b3"), false), (String::from("a3$e"), false)];

  for (word, expected) in test_cases {
    assert_eq!(Solution::is_valid(word), expected);
  }
}
