fn solve(n: usize, arr: &mut [i64]) -> i64 {
  let mut ans: i64 = 0;

  for i in 1..n {
    let diff: i64 = arr[i] - arr[i - 1];
    if diff < 0 {
      ans -= diff;
      arr[i] = arr[i - 1];
    }
  }

  ans
}
