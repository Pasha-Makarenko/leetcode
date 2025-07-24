// 9. Palindrome Number

struct Solution;

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
      return false;
    }

    let mut reversed = 0;
    let mut num = x;

    while num > reversed {
      reversed = reversed * 10 + num % 10;
      num /= 10;
    }

    num == reversed || num == reversed / 10
  }
}

#[test]
fn test() {
  let test_cases = [(121, true), (-121, false), (10, false)];

  for (x, expected) in test_cases {
    assert_eq!(Solution::is_palindrome(x), expected);
  }
}
