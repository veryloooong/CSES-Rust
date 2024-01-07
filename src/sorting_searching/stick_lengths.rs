use std::cmp::min;

fn solve(arr: &mut [isize]) -> isize {
  let n = arr.len();
  let mut sum = arr.iter().sum();

  arr.sort_unstable();
  sum -= arr[0] * n as isize;

  for i in 0..n - 1 {
    sum = min(
      sum,
      sum + (arr[i + 1] - arr[i]) * (i - (n - i - 2)) as isize,
    );
  }

  sum
}
