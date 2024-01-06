fn solve(n: u64) -> u64 {
  let mut ans: u64 = 0;

  let mut i: u64 = 5;

  while n / i >= 1 {
    ans += n / i;
    i *= 5;
  }

  ans
}
