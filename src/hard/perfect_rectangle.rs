// 391. Perfect Rectangle

use std::collections::HashSet;

struct Solution;

impl Solution {
  pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
    let (mut x1, mut y1, mut x2, mut y2) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    let mut area = 0;
    let mut corners = HashSet::new();

    for rect in &rectangles {
      let (rx1, ry1, rx2, ry2) = (rect[0], rect[1], rect[2], rect[3]);
      area += (rx2 - rx1) as i64 * (ry2 - ry1) as i64;

      x1 = x1.min(rx1);
      y1 = y1.min(ry1);
      x2 = x2.max(rx2);
      y2 = y2.max(ry2);

      for &(x, y) in &[(rx1, ry1), (rx1, ry2), (rx2, ry1), (rx2, ry2)] {
        if !corners.insert((x, y)) {
          corners.remove(&(x, y));
        }
      }
    }

    corners == HashSet::from([(x1, y1), (x1, y2), (x2, y1), (x2, y2)])
      && area == (x2 - x1) as i64 * (y2 - y1) as i64
  }
}

#[test]
fn test() {
  let test_cases = [
    (
      vec![
        vec![1, 1, 3, 3],
        vec![3, 1, 4, 2],
        vec![3, 2, 4, 4],
        vec![1, 3, 2, 4],
        vec![2, 3, 3, 4],
      ],
      true,
    ),
    (
      vec![vec![1, 1, 2, 3], vec![1, 3, 2, 4], vec![3, 1, 4, 2], vec![3, 2, 4, 4]],
      false,
    ),
    (
      vec![vec![1, 1, 3, 3], vec![3, 1, 4, 2], vec![1, 3, 2, 4], vec![2, 2, 4, 4]],
      false,
    ),
  ];

  for (rectangles, expected) in test_cases {
    assert_eq!(Solution::is_rectangle_cover(rectangles), expected);
  }
}
