// 1717. Maximum Score From Removing Substrings

struct Solution;

impl Solution {
  fn calculate(sub: &str, x: i32, y: i32) -> i32 {
    let (first, second, first_points, second_points) =
      if x >= y { ('a', 'b', x, y) } else { ('b', 'a', y, x) };

    let (points1, remaining) = Self::remove_pairs(sub, first, second, first_points);
    let (points2, _) = Self::remove_pairs(&remaining, second, first, second_points);

    points1 + points2
  }

  fn remove_pairs(s: &str, first: char, second: char, points: i32) -> (i32, String) {
    let mut stack = Vec::with_capacity(s.len());
    let mut score = 0;

    for c in s.chars() {
      if !stack.is_empty() && stack.last() == Some(&first) && c == second {
        stack.pop();
        score += points;
      } else {
        stack.push(c);
      }
    }

    (score, stack.into_iter().collect())
  }

  pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    let mut result = 0;
    let mut current_sub = String::new();

    for c in s.chars() {
      if c == 'a' || c == 'b' {
        current_sub.push(c);
      } else if !current_sub.is_empty() {
        result += Self::calculate(&current_sub, x, y);
        current_sub.clear();
      }
    }

    if !current_sub.is_empty() {
      result += Self::calculate(&current_sub, x, y);
    }

    result
  }
}

#[test]
fn test() {
  let test_cases =
    [(String::from("cdbcbbaaabab"), 4, 5, 19), (String::from("aabbaaxybbaabb"), 5, 4, 20)];

  for (s, x, y, expected) in test_cases {
    assert_eq!(Solution::maximum_gain(s, x, y), expected);
  }
}
