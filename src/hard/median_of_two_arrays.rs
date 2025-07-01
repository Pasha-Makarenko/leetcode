// 4. Median of Two Sorted Arrays

struct Solution;

impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (first, second) = if nums1.len() <= nums2.len() { (nums1, nums2) } else { (nums2, nums1) };
    let (n, m) = (first.len(), second.len());
    let total = n + m;
    let half = total.div_ceil(2);
    let (mut left, mut right) = (0, n);

    while left <= right {
      let i = (right + left) / 2;
      let j = half - i;

      if i > 0 && j < m && first[i - 1] > second[j] {
        right = i - 1;
      } else if j > 0 && i < n && second[j - 1] > first[i] {
        left = i + 1;
      } else {
        let max_left = if i == 0 {
          second[j - 1]
        } else if j == 0 {
          first[i - 1]
        } else {
          first[i - 1].max(second[j - 1])
        };

        if total % 2 == 1 {
          return max_left as f64;
        }

        let min_right = if i == n {
          second[j]
        } else if j == m {
          first[i]
        } else {
          first[i].min(second[j])
        };

        return (max_left + min_right) as f64 / 2.0;
      }
    }

    panic!("No value")
  }
}

#[test]
fn test() {
  let test_cases = [(vec![1, 3], vec![2], 2.00000), (vec![1, 2], vec![3, 4], 2.50000)];

  for (nums1, nums2, expected) in test_cases {
    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
  }
}
