// 1233. Remove Sub-Folders from the Filesystem

struct Solution;

impl Solution {
  pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
    let mut folder = std::mem::take(&mut folder);
    folder.sort_unstable();

    let mut result = Vec::<String>::with_capacity(folder.len());

    for path in folder {
      let should_push = match result.last() {
        None => true,
        Some(last) => {
          let last_len = last.len();
          path.len() <= last_len
            || !path.starts_with(last)
            || unsafe { *path.as_bytes().get_unchecked(last_len) != b'/' }
        }
      };

      if should_push {
        result.push(path);
      }
    }

    result
  }
}

#[test]
fn test() {
  let test_cases = [
    (vec!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"], vec!["/a", "/c/d", "/c/f"]),
    (vec!["/a", "/a/b/c", "/a/b/d"], vec!["/a"]),
    (vec!["/a/b/c", "/a/b/ca", "/a/b/d"], vec!["/a/b/c", "/a/b/ca", "/a/b/d"]),
  ];

  for (folder, expected) in test_cases {
    assert_eq!(
      Solution::remove_subfolders(folder.into_iter().map(String::from).collect::<Vec<String>>()),
      expected.into_iter().map(String::from).collect::<Vec<String>>()
    );
  }
}
