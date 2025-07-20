// 1948. Delete Duplicate Folders in System

use std::collections::{BTreeMap, HashMap};

struct Node {
  deleted: bool,
  children: BTreeMap<String, Node>,
}

impl Node {
  fn new() -> Self {
    Node { deleted: false, children: BTreeMap::new() }
  }
}

struct Solution;

impl Solution {
  fn serialize(node: &Node, seen: &mut HashMap<String, Vec<*mut Node>>) -> String {
    let child_keys = node
      .children
      .iter()
      .map(|(name, child)| format!("{}{}", name, Self::serialize(child, seen)))
      .collect::<String>();

    let key = format!("({child_keys})");

    if !node.children.is_empty() {
      seen.entry(key.clone()).or_default().push(node as *const _ as *mut _);
    }

    key
  }

  fn collect(node: &Node, path: &mut Vec<String>, paths: &mut Vec<Vec<String>>) {
    for (name, child) in &node.children {
      if !child.deleted {
        path.push(name.clone());
        paths.push(path.clone());
        Self::collect(child, path, paths);
        path.pop();
      }
    }
  }

  pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut node = Node::new();

    for path in paths {
      let mut current = &mut node;
      for part in path {
        current = current.children.entry(part.clone()).or_insert_with(Node::new);
      }
    }

    let mut seen = HashMap::new();
    Self::serialize(&node, &mut seen);

    for nodes in seen.values() {
      if nodes.len() > 1 {
        for &node in nodes {
          unsafe {
            (*node).deleted = true;
          }
        }
      }
    }

    let mut result = Vec::new();
    let mut path = Vec::new();
    Self::collect(&node, &mut path, &mut result);

    result
  }
}

#[test]
fn test() {
  let test_cases = vec![
    (
      vec![vec!["a"], vec!["c"], vec!["d"], vec!["a", "b"], vec!["c", "b"], vec!["d", "a"]],
      vec![vec!["d"], vec!["d", "a"]],
    ),
    (
      vec![
        vec!["a"],
        vec!["c"],
        vec!["a", "b"],
        vec!["c", "b"],
        vec!["a", "b", "x"],
        vec!["a", "b", "x", "y"],
        vec!["w"],
        vec!["w", "y"],
      ],
      vec![vec!["a"], vec!["a", "b"], vec!["c"], vec!["c", "b"]],
    ),
    (
      vec![vec!["a", "b"], vec!["c", "d"], vec!["c"], vec!["a"]],
      vec![vec!["a"], vec!["a", "b"], vec!["c"], vec!["c", "d"]],
    ),
  ];

  for (paths, expected) in test_cases {
    assert_eq!(
      Solution::delete_duplicate_folder(
        paths
          .into_iter()
          .map(|inner_vec| inner_vec.into_iter().map(String::from).collect::<Vec<String>>())
          .collect::<Vec<Vec<String>>>()
      ),
      expected
        .into_iter()
        .map(|inner_vec| inner_vec.into_iter().map(String::from).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
    );
  }
}
