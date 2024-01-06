const MOD: usize = 1_000_000_007;

fn solve(n: usize) -> usize {
  let mut ans = 1;

  let _ = (0..n).map(|_| {
    ans = (ans << 1) % MOD;
  });

  ans
}
