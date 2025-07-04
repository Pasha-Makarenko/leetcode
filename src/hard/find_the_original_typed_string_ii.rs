// 3333. Find the Original Typed String II

struct Solution;

impl Solution {
  pub fn possible_string_count(word: String, k: i32) -> i32 {
    let mod_val: i32 = 1_000_000_007;
    let mut groups: Vec<i32> = Vec::new();
    let mut chars = word.chars().peekable();

    while let Some(c) = chars.next() {
      let mut count = 1;

      while chars.peek() == Some(&c) {
        count += 1;
        chars.next();
      }

      groups.push(count);
    }

    let mut total_combinations: i32 = 1;
    for &freq in &groups {
      total_combinations = ((total_combinations as i64) * freq as i64 % mod_val as i64) as i32;
    }

    if groups.len() as i32 >= k {
      return total_combinations;
    }

    let k_usize = k as usize;
    let mut dp: Vec<i32> = vec![0; k_usize];
    let mut new_dp: Vec<i32> = vec![0; k_usize];
    let mut prefix_sum: Vec<i32> = vec![0; k_usize];

    dp[0] = 1;

    for &freq in &groups {
      prefix_sum[0] = dp[0];
      for j in 1..k_usize {
        prefix_sum[j] = (prefix_sum[j - 1] + dp[j]) % mod_val;
      }

      for j in 1..k_usize {
        let right = prefix_sum[j - 1];
        let mut left = 0;
        if (j as i32 - 1 - freq) >= 0 {
          left = prefix_sum[(j as i32 - 1 - freq) as usize];
        }
        new_dp[j] = (right - left + mod_val) % mod_val;
      }

      std::mem::swap(&mut dp, &mut new_dp);
      new_dp.iter_mut().for_each(|x| *x = 0);
    }

    let mut invalid_combinations: i32 = 0;
    for comb in dp.iter().take(k_usize) {
      invalid_combinations = (invalid_combinations + comb) % mod_val;
    }

    (total_combinations - invalid_combinations + mod_val) % mod_val
  }
}

#[test]
fn test() {
  let test_cases = [
    (String::from("aabbccdd"), 7, 5),
    (String::from("aabbccdd"), 8, 1),
    (String::from("aaabbb"), 3, 8),
  ];

  for (word, k, expected) in test_cases {
    assert_eq!(Solution::possible_string_count(word, k), expected);
  }
}
