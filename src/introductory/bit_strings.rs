const MOD: usize = 1_000_000_007;

fn solve(n: usize) -> usize {
  let mut ans = 1;

  (0..n).for_each(|_| {
    ans = (ans << 1) % MOD;
  });

  ans
}
