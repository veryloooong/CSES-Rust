use std::io::{stdout, BufWriter, Write};

fn solve(n: usize, queries: &[i64]) {
  let mut output = BufWriter::new(stdout().lock());

  for &x in queries.iter().take(n) {
    let mut ans: i64 = 0;

    let mut d = 1;
    while d * d < x {
      if x % d == 0 {
        ans += 2;
      }
      d += 1;
    }
    if d * d == x {
      ans += 1;
    }

    writeln!(output, "{ans}").ok();
  }
}
