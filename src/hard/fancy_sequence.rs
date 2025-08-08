// 1622. Fancy Sequence

const MODULO: i64 = 1_000_000_007;

struct Fancy {
  items: Vec<i64>,
  add: i64,
  mult: i64,
}

impl Fancy {
  fn new() -> Self {
    Fancy { items: Vec::new(), add: 0, mult: 1 }
  }

  fn append(&mut self, val: i32) {
    self.items.push(((val as i64 - self.add) * Self::mod_inv(self.mult, MODULO)) % MODULO);
  }

  fn add_all(&mut self, inc: i32) {
    self.add = (self.add + inc as i64) % MODULO;
  }

  fn mult_all(&mut self, m: i32) {
    self.add = (self.add * m as i64) % MODULO;
    self.mult = (self.mult * m as i64) % MODULO;
  }

  fn get_index(&self, idx: i32) -> i32 {
    if (idx as usize) < self.items.len() {
      let res = (self.mult * self.items[idx as usize] + self.add) % MODULO;
      (if res < 0 { res + MODULO } else { res }) as i32
    } else {
      -1
    }
  }

  fn mod_inv(value: i64, modulo: i64) -> i64 {
    let mut res: i64 = 1;
    let mut base = (value % modulo + modulo) % modulo;
    let mut power = modulo - 2;

    while power > 0 {
      if power & 1 == 1 {
        res = res * base % modulo;
      }
      base = base * base % modulo;
      power >>= 1;
    }

    res
  }
}

#[test]
fn test() {
  let test_cases = [(
    vec![
      "append", "addAll", "append", "multAll", "getIndex", "addAll", "append", "multAll",
      "getIndex", "getIndex", "getIndex",
    ],
    vec![[2], [3], [7], [2], [0], [3], [10], [2], [0], [1], [2]],
    vec![None, None, None, None, Some(10), None, None, None, Some(26), Some(34), Some(20)],
  )];

  for (ops, args, expected) in test_cases {
    let mut fancy = Fancy::new();

    for (i, &op) in ops.iter().enumerate() {
      match op {
        "append" => fancy.append(args[i][0]),
        "addAll" => fancy.add_all(args[i][0]),
        "multAll" => fancy.mult_all(args[i][0]),
        "getIndex" => assert_eq!(fancy.get_index(args[i][0]), expected[i].unwrap()),
        _ => {}
      }
    }
  }
}
