fn solve(str: String) -> u64 {
  let n = str.len();
  let mut max: u64 = 1;
  let mut cur: u64 = 1;

  let str = str.into_bytes();

  for i in 1..n {
    if str[i] == str[i - 1] {
      cur += 1;
    } else {
      cur = 1;
    }
    max = std::cmp::max(max, cur);
  }

  max
}
