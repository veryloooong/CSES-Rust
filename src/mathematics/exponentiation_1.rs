use std::io::{stdout, BufWriter, Write};

const MOD: u64 = 1_000_000_007;

fn main(n: usize, arr: &[(u64, u64)]) {
  let mut out = BufWriter::new(stdout().lock());

  for &(a, b) in arr.iter().take(n) {
    let (mut a, mut b) = (a, b);

    let mut ans: u64 = 1;

    while b > 0 {
      if b % 2 == 1 {
        ans = ans * a % MOD;
      }
      a = a * a % MOD;
      b /= 2;
    }

    writeln!(out, "{ans}").ok();
  }
}
