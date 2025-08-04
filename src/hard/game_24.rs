// 679. 24 Game

struct Solution;

impl Solution {
  fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
  }

  fn permutations<T: Clone>(arr: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current = arr.to_vec();
    let n = arr.len();

    fn generate<T: Clone>(n: usize, current: &mut Vec<T>, result: &mut Vec<Vec<T>>) {
      if n == 1 {
        result.push(current.clone());
        return;
      }

      for i in 0..n {
        generate(n - 1, current, result);

        if n % 2 == 0 {
          current.swap(i, n - 1);
        } else {
          current.swap(0, n - 1);
        }
      }
    }

    generate(n, &mut current, &mut result);
    result
  }

  fn process(cards: &[f64], ops: &[i32]) -> bool {
    if cards.len() == 1 {
      return Self::approx_equal(cards[0], 24f64, 1e-6);
    }

    let mut result = false;

    for i in 0..cards.len() - 1 {
      let mut new_cards = cards.to_owned();
      let removed = new_cards.remove(i + 1);
      let mut new_ops = ops.to_owned();
      let current_op = new_ops.remove(i);

      match current_op {
        1 => new_cards[i] += removed,
        2 => new_cards[i] -= removed,
        3 => new_cards[i] *= removed,
        4 => new_cards[i] /= removed,
        _ => {}
      }

      result = result || Self::process(&new_cards, &new_ops);

      if result {
        return result;
      }
    }

    result
  }

  pub fn judge_point24(cards: Vec<i32>) -> bool {
    let cards: Vec<f64> = cards.into_iter().map(|c| c as f64).collect();
    let perms = Self::permutations(&cards);
    let ops_list: Vec<Vec<i32>> = vec![
      vec![1, 1, 1],
      vec![1, 1, 2],
      vec![1, 1, 3],
      vec![1, 1, 4],
      vec![1, 2, 1],
      vec![1, 2, 2],
      vec![1, 2, 3],
      vec![1, 2, 4],
      vec![1, 3, 1],
      vec![1, 3, 2],
      vec![1, 3, 3],
      vec![1, 3, 4],
      vec![1, 4, 1],
      vec![1, 4, 2],
      vec![1, 4, 3],
      vec![1, 4, 4],
      vec![2, 1, 1],
      vec![2, 1, 2],
      vec![2, 1, 3],
      vec![2, 1, 4],
      vec![2, 2, 1],
      vec![2, 2, 2],
      vec![2, 2, 3],
      vec![2, 2, 4],
      vec![2, 3, 1],
      vec![2, 3, 2],
      vec![2, 3, 3],
      vec![2, 3, 4],
      vec![2, 4, 1],
      vec![2, 4, 2],
      vec![2, 4, 3],
      vec![2, 4, 4],
      vec![3, 1, 1],
      vec![3, 1, 2],
      vec![3, 1, 3],
      vec![3, 1, 4],
      vec![3, 2, 1],
      vec![3, 2, 2],
      vec![3, 2, 3],
      vec![3, 2, 4],
      vec![3, 3, 1],
      vec![3, 3, 2],
      vec![3, 3, 3],
      vec![3, 3, 4],
      vec![3, 4, 1],
      vec![3, 4, 2],
      vec![3, 4, 3],
      vec![3, 4, 4],
      vec![4, 1, 1],
      vec![4, 1, 2],
      vec![4, 1, 3],
      vec![4, 1, 4],
      vec![4, 2, 1],
      vec![4, 2, 2],
      vec![4, 2, 3],
      vec![4, 2, 4],
      vec![4, 3, 1],
      vec![4, 3, 2],
      vec![4, 3, 3],
      vec![4, 3, 4],
      vec![4, 4, 1],
      vec![4, 4, 2],
      vec![4, 4, 3],
      vec![4, 4, 4],
    ];

    let mut result = false;

    for perm in &perms {
      for ops in &ops_list {
        result = result || Self::process(perm, ops);
        if result {
          return result;
        }
      }
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [(vec![4, 1, 8, 7], true), (vec![1, 2, 1, 2], false)];

  for (cards, expected) in test_cases {
    assert_eq!(Solution::judge_point24(cards), expected);
  }
}
