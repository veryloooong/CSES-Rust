fn solve(n: usize, arr: &[usize]) -> usize {
  n * (n - 1) / 2 - arr.iter().take(n - 1).sum::<usize>()
}
