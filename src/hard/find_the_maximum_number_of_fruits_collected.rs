// 3363. Find the Maximum Number of Fruits Collected

struct Solution;

impl Solution {
  fn transpose(matrix: &mut [Vec<i32>]) {
    for i in 0..matrix.len() {
      for j in 0..i {
        let temp = matrix[i][j];
        matrix[i][j] = matrix[j][i];
        matrix[j][i] = temp;
      }
    }
  }

  fn max_path(matrix: &[Vec<i32>]) -> i32 {
    let n = matrix.len();
    let mut prev = vec![i32::MIN; n];
    let mut curr = vec![i32::MIN; n];

    prev[n - 1] = matrix[0][n - 1];

    for (i, row) in matrix.iter().enumerate().take(n - 1).skip(1) {
      for j in (n - 1 - i).max(i + 1)..n {
        curr[j] = row[j]
          + prev[j]
            .max(*prev.get(j - 1).unwrap_or(&i32::MIN))
            .max(*prev.get(j + 1).unwrap_or(&i32::MIN));
      }

      prev = curr.clone();
    }

    prev[n - 1]
  }

  pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
    let n = fruits.len();
    let mut total: i32 = (0..n).map(|i| fruits[i][i]).sum();

    total += Self::max_path(&fruits);
    Self::transpose(&mut fruits);
    total += Self::max_path(&fruits);

    total
  }
}

#[test]
fn test() {
  let test_cases = [
    (
      vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]],
      100,
    ),
    (vec![vec![1, 1], vec![1, 1]], 4),
  ];

  for (fruits, expected) in test_cases {
    assert_eq!(Solution::max_collected_fruits(fruits), expected);
  }
}

// Using dijkstra algorithm instead of dynamic programming

// use std::collections::{BinaryHeap, HashMap};
// use std::cmp::Ordering;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// struct Point {
//   i: usize,
//   j: usize
// }

// #[derive(Debug, Clone, Copy)]
// struct HeapItem {
//   dist: i32,
//   point: Point
// }

// impl Ord for HeapItem {
//   fn cmp(&self, other: &Self) -> Ordering {
//     self.dist.cmp(&other.dist)
//   }
// }

// impl PartialOrd for HeapItem {
//   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//     Some(self.cmp(other))
//   }
// }

// impl PartialEq for HeapItem {
//   fn eq(&self, other: &Self) -> bool {
//     self.dist == other.dist
//   }
// }

// impl Eq for HeapItem {}

// pub fn get_upper_triangle_neighbors(p: Point, n: usize) -> Vec<Point> {
//   let mut neighbors = Vec::new();

//   if p.i == n - 1 && p.j == n - 1 {
//     return neighbors;
//   }

//   let directions = [
//     (1, 1),
//     (1, 0),
//     (1, -1)
//   ];

//   for (di, dj) in directions.iter() {
//     let new_i = p.i as i32 + di;
//     let new_j = p.j as i32 + dj;

//     if new_i < 0 || new_j < 0 || new_i >= n as i32 || new_j >= n as i32 || new_i >= new_j && !(new_i == n as i32 - 1 && new_j == n as i32 - 1) {
//       continue;
//     }

//     neighbors.push(Point { i: new_i as usize, j: new_j as usize });
//   }

//   neighbors
// }

// pub fn get_lower_triangle_neighbors(p: Point, n: usize) -> Vec<Point> {
//   let mut neighbors = Vec::new();

//   if p.i == n - 1 && p.j == n - 1 {
//     return neighbors;
//   }

//   let directions = [
//     (1, 1),
//     (0, 1),
//     (-1, 1)
//   ];

//   for (di, dj) in directions.iter() {
//     let new_i = p.i as i32 + di;
//     let new_j = p.j as i32 + dj;

//     if new_i < 0 || new_j < 0 || new_i >= n as i32 || new_j >= n as i32 || new_i <= new_j && !(new_i == n as i32 - 1 && new_j == n as i32 - 1) {
//       continue;
//     }

//     neighbors.push(Point { i: new_i as usize, j: new_j as usize });
//   }

//   neighbors
// }

// pub fn dijkstra_max_path(
//   matrix: &[Vec<i32>],
//   start: Point,
//   get_neighbors: impl Fn(Point) -> Vec<Point>
// ) -> HashMap<Point, i32> {
//   let mut distances = HashMap::new();
//   let mut heap = BinaryHeap::new();

//   distances.insert(start, matrix[start.i][start.j]);
//   heap.push(HeapItem {
//     dist: matrix[start.i][start.j],
//     point: start
//   });

//   while let Some(HeapItem { dist: current_dist, point: current_point }) = heap.pop() {
//     if current_dist < *distances.get(&current_point).unwrap_or(&i32::MIN) {
//       continue;
//     }

//     let neighbors = get_neighbors(current_point);

//     for neighbor in neighbors {
//       let new_dist = current_dist + matrix[neighbor.i][neighbor.j];

//       if new_dist > *distances.get(&neighbor).unwrap_or(&i32::MIN) {
//         distances.insert(neighbor, new_dist);
//         heap.push(HeapItem {
//           dist: new_dist,
//           point: neighbor
//         });
//       }
//     }
//   }

//   distances
// }

// struct Solution;

// impl Solution {
//   pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
//     let n = fruits.len();

//     let start_upper = Point { i: 0, j: n - 1 };
//     let distances_upper = dijkstra_max_path(&fruits, start_upper, |p| get_upper_triangle_neighbors(p, n));
//     let upper_path = *distances_upper.get(&Point { i: n - 1, j: n - 1 }).unwrap_or(&0);

//     let start_lower = Point { i: n - 1, j: 0 };
//     let distances_lower = dijkstra_max_path(&fruits, start_lower, |p| get_lower_triangle_neighbors(p, n));
//     let lower_path = *distances_lower.get(&Point { i: n - 1, j: n - 1 }).unwrap_or(&0);

//     let diagonal_sum: i32 = (0..n).map(|i| fruits[i][i]).sum();
//     let last_diag = fruits[n - 1][n - 1];

//     diagonal_sum + upper_path + lower_path - 2 * last_diag
//   }
// }
