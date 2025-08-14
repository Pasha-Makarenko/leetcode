// 2264. Largest 3-Same-Digit Number in String

const CHECKS: [&str; 10] = ["999", "888", "777", "666", "555", "444", "333", "222", "111", "000"];

struct Solution;

impl Solution {
  pub fn largest_good_integer(num: String) -> String {
    for &check in CHECKS.iter() {
      if num.contains(check) {
        return check.to_string();
      }
    }

    String::new()
  }
}

#[test]
fn test() {
  let test_cases = [
    (String::from("6777133339"), String::from("777")),
    (String::from("2300019"), String::from("000")),
    (String::from("42352338"), String::from("")),
  ];

  for (num, expected) in test_cases {
    assert_eq!(Solution::largest_good_integer(num), expected);
  }
}
