use std::cmp::min;

const MOD: i64 = 1_000_000_007;

fn solve(n: usize) -> i64 {
  let mut arr: Vec<i64> = Vec::with_capacity(n + 1);

  arr.push(1);

  for _ in 0..n {
    arr.push(0);
  }

  for a in 1..=n {
    for b in 1..=min(6, a) {
      arr[a] = (arr[a] + arr[a - b]) % MOD;
    }
  }

  arr[n]
}
