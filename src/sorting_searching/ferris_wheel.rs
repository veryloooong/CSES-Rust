fn main(n: usize, x: u64, arr: &mut [u64]) -> usize {
  arr.sort_unstable();

  let mut lhs: usize = 0;
  let mut rhs: usize = n - 1;
  let mut ans = 0;

  while lhs <= rhs && rhs < n {
    if arr[lhs] + arr[rhs] <= x {
      lhs += 1;
    }
    ans += 1;
    rhs -= 1;
  }

  ans
}
