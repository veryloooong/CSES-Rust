use crate::default_io;

use std::cmp::max;
use std::collections::HashMap;

fn solve() -> usize {
  let (mut input, _) = default_io();

  let n: usize = input.token();

  let mut l = 1;
  let mut ans = 0;

  let mut songs: HashMap<usize, usize> = HashMap::new();

  for r in 1..=n {
    let p: usize = input.token();
    if let Some(&t) = songs.get(&p) {
      ans = max(ans, r - l);
      l = max(l, t + 1);
      *songs.get_mut(&p).unwrap() = r;
    } else {
      ans = max(ans, r - l + 1);
      songs.insert(p, r);
    }
  }

  ans = max(ans, n - l + 1);

  ans
}
