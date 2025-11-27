// 3163. String Compression III

struct Solution;

impl Solution {
  pub fn compressed_string(word: String) -> String {
    let mut comp = Vec::with_capacity(word.len());
    let mut i = 0;
    let chars = word.as_bytes();

    while i < chars.len() {
      let current_char = chars[i];
      let mut count = 0;

      while i < chars.len() && current_char == chars[i] && count < 9 {
        count += 1;
        i += 1;
      }

      comp.push(b'0' + count);
      comp.push(current_char);
    }

    unsafe { String::from_utf8_unchecked(comp) }
  }
}

#[test]
fn test() {
  let test_cases = [
    (String::from("abcde"), String::from("1a1b1c1d1e")),
    (String::from("aaaaaaaaaaaaaabb"), String::from("9a5a2b")),
  ];

  for (word, expected) in test_cases {
    assert_eq!(Solution::compressed_string(word), expected);
  }
}
