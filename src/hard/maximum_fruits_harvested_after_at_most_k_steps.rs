// 2106. Maximum Fruits Harvested After at Most K Steps

struct Solution;

impl Solution {
  pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
    let n = fruits.len();
    let (mut l, mut r) = (0, n);

    while l < r {
      let mid = (l + r) / 2;
      if fruits[mid][0] < start_pos - k {
        l = mid + 1;
      } else {
        r = mid;
      }
    }

    let left_bound = l;
    r = n;

    while l < r {
      let mid = (l + r) / 2;
      if fruits[mid][0] <= start_pos + k {
        l = mid + 1;
      } else {
        r = mid;
      }
    }
    let right_bound = r;

    if left_bound >= right_bound {
      return 0;
    }

    let mut max = 0;
    let (mut sum, mut j) = (0, left_bound);

    for i in left_bound..right_bound {
      sum += fruits[i][1];
      while j <= i && (fruits[i][0] - start_pos) * 2 + (start_pos - fruits[j][0]) > k {
        sum -= fruits[j][1];
        j += 1;
      }
      max = max.max(sum);
    }

    sum = 0;
    j = right_bound - 1;

    for i in (left_bound..right_bound).rev() {
      sum += fruits[i][1];
      while j >= i && (start_pos - fruits[i][0]) * 2 + (fruits[j][0] - start_pos) > k {
        sum -= fruits[j][1];
        j -= 1;
      }
      max = max.max(sum);
    }

    max
  }
}

#[test]
fn test() {
  let test_cases = [
    (vec![vec![2, 8], vec![6, 3], vec![8, 6]], 5, 4, 9),
    (
      vec![vec![0, 9], vec![4, 1], vec![5, 7], vec![6, 2], vec![7, 4], vec![10, 9]],
      5,
      4,
      14,
    ),
    (vec![vec![0, 3], vec![6, 4], vec![8, 5]], 3, 2, 0),
  ];

  for (fruits, start_pos, k, expected) in test_cases {
    assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), expected);
  }
}
