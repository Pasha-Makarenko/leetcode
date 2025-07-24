// 2322. Minimum Score After Removals on a Tree

struct Solution;

impl Solution {
  fn dfs(
    u: usize,
    parent: usize,
    adj: &Vec<Vec<usize>>,
    nums: &Vec<i32>,
    in_time: &mut Vec<i32>,
    out_time: &mut Vec<i32>,
    sub_xor: &mut Vec<i32>,
    timer: &mut i32,
  ) {
    *timer += 1;
    in_time[u] = *timer;
    sub_xor[u] = nums[u];

    for &v in adj[u].iter() {
      if v != parent {
        Self::dfs(v, u, adj, nums, in_time, out_time, sub_xor, timer);
        sub_xor[u] ^= sub_xor[v];
      }
    }

    out_time[u] = *timer;
  }

  pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut adj = vec![Vec::new(); n];
    let mut in_time = vec![0; n];
    let mut out_time = vec![0; n];
    let mut sub_xor = vec![0; n];
    let mut total_xor = 0;
    let mut timer = 0;
    let root = 0;

    for &num in &nums {
      total_xor ^= num;
    }

    for edge in &edges {
      let (u, v) = (edge[0] as usize, edge[1] as usize);
      adj[u].push(v);
      adj[v].push(u);
    }

    Self::dfs(root, root, &adj, &nums, &mut in_time, &mut out_time, &mut sub_xor, &mut timer);

    let mut score = i32::MAX;

    for (i, edge1) in edges.iter().enumerate() {
      let (u1, v1) = (edge1[0] as usize, edge1[1] as usize);
      let child1 = if in_time[u1] < in_time[v1] { v1 } else { u1 };
      let xor1 = sub_xor[child1];

      for edge2 in edges.iter().skip(i + 1) {
        let (u2, v2) = (edge2[0] as usize, edge2[1] as usize);
        let child2 = if in_time[u2] < in_time[v2] { v2 } else { u2 };
        let xor2 = sub_xor[child2];

        let (c1, c2, c3) =
          if in_time[child1] <= in_time[child2] && out_time[child2] <= out_time[child1] {
            (sub_xor[child2], xor1 ^ sub_xor[child2], total_xor ^ xor1)
          } else if in_time[child2] <= in_time[child1] && out_time[child1] <= out_time[child2] {
            (sub_xor[child1], xor2 ^ sub_xor[child1], total_xor ^ xor2)
          } else {
            (xor1, xor2, total_xor ^ xor1 ^ xor2)
          };

        score = score.min(c1.max(c2).max(c3) - c1.min(c2).min(c3));
      }
    }

    score
  }
}

#[test]
fn test() {
  let test_cases = [
    (vec![1, 5, 5, 4, 11], vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]], 9),
    (
      vec![5, 5, 2, 4, 4, 2],
      vec![vec![0, 1], vec![1, 2], vec![5, 2], vec![4, 3], vec![1, 3]],
      0,
    ),
  ];

  for (nums, edges, expected) in test_cases {
    assert_eq!(Solution::minimum_score(nums, edges), expected);
  }
}
