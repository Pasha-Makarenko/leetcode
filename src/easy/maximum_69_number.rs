// 1323. Maximum 69 Number

struct Solution;

impl Solution {
  pub fn maximum69_number(num: i32) -> i32 {
    let mut pow = -1i32;
    let mut i = 0;
    let mut temp = num;

    while temp > 0 {
      let digit = temp % 10;

      if digit == 6 {
        pow = i;
      }

      temp /= 10;
      i += 1;
    }

    if pow == -1 {
      num
    } else {
      num + 3 * 10_i32.pow(pow as u32)
    }
  }
}

#[test]
fn test() {
  let test_cases = [(9669, 9969), (9996, 9999), (9999, 9999)];

  for (num, expected) in test_cases {
    assert_eq!(Solution::maximum69_number(num), expected);
  }
}
