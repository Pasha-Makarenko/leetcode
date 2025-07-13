// 68. Text Justification

const SPACE: char = ' ';

struct Solution;

impl Solution {
  pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let words_count = words.len();
    let mut lines = vec![(0, 0, 0)];
    let mut group_index = 0;

    for (i, word) in words.iter().enumerate() {
      let len = word.len();

      if lines[group_index].2 + len <= max_width as usize {
        lines[group_index].1 += 1;
        lines[group_index].2 += len + 1;
      } else {
        lines.push((i, i + 1, len + 1));
        group_index += 1;
      }
    }

    let mut result = Vec::new();

    for (start, end, sum) in lines {
      let mut line = String::from("");
      let count = (end - start) as i32;

      if count != 1 {
        let spaces = max_width - sum as i32 + count;
        let (div, offset) =
          if end == words_count { (1, 0) } else { (spaces / (count - 1), spaces % (count - 1)) };

        for (i, word) in words.iter().enumerate().take(end).skip(start) {
          line.push_str(word);

          if offset == 0 {
            if i < end - 1 {
              for _ in 0..div {
                line.push(SPACE)
              }
            }
          } else if i < end - (count - offset) as usize {
            for _ in 0..=div {
              line.push(SPACE)
            }
          } else if i < end - 1 {
            for _ in 0..div {
              line.push(SPACE)
            }
          }
        }
      } else {
        line.push_str(&words[start]);
      }

      if end == words_count || count == 1 {
        while line.len() < max_width as usize {
          line.push(SPACE);
        }
      }

      result.push(line);
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [
    (
      vec![
        String::from("This"),
        String::from("is"),
        String::from("an"),
        String::from("example"),
        String::from("of"),
        String::from("text"),
        String::from("justification."),
      ],
      16,
      vec![
        String::from("This    is    an"),
        String::from("example  of text"),
        String::from("justification.  "),
      ],
    ),
    (
      vec![
        String::from("What"),
        String::from("must"),
        String::from("be"),
        String::from("acknowledgment"),
        String::from("shall"),
        String::from("be"),
      ],
      16,
      vec![
        String::from("What   must   be"),
        String::from("acknowledgment  "),
        String::from("shall be        "),
      ],
    ),
  ];

  for (words, max_width, expected) in test_cases {
    assert_eq!(Solution::full_justify(words, max_width), expected);
  }
}
