fn solve(n: usize) -> Vec<usize> {
  (1..=n)
    .map(|i| {
      let a = i * i;
      a * (a - 1) / 2 - 4 * (i - 1) * (i - 2)
    })
    .collect()
}
