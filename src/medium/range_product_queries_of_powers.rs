// 2438. Range Product Queries of Powers

const MODULO: i64 = 1_000_000_007;

struct Solution;

impl Solution {
  pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut n = n;
    let mut powers = Vec::new();
    let mut power = 1;

    while n > 0 {
      if n & 1 == 1 {
        powers.push(power as i64);
      }
      power <<= 1;
      n >>= 1;
    }

    let mut prefix = vec![1i64];

    for &power in powers.iter() {
      let last = *prefix.last().unwrap();
      prefix.push((power * last) % MODULO);
    }

    let mut answers = Vec::new();

    for query in queries.iter() {
      let (left, right) = (prefix[query[0] as usize], prefix[query[1] as usize + 1]);

      answers.push(((right * Self::mod_inv(left, MODULO)) % MODULO) as i32);
    }

    answers
  }

  fn mod_inv(value: i64, modulo: i64) -> i64 {
    let mut res: i64 = 1;
    let mut base = (value % modulo + modulo) % modulo;
    let mut power = modulo - 2;

    while power > 0 {
      if power & 1 == 1 {
        res = (res * base) % modulo;
      }
      base = (base * base) % modulo;
      power >>= 1;
    }

    res
  }
}

#[test]
fn test() {
  let test_cases = [
    (15, vec![vec![0, 1], vec![2, 2], vec![0, 3]], vec![2, 4, 64]),
    (2, vec![vec![0, 0]], vec![2]),
  ];

  for (n, queries, expected) in test_cases {
    assert_eq!(Solution::product_queries(n, queries), expected);
  }
}
