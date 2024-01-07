fn solve(arr: &[usize]) -> usize {
  let mut ans = 1;

  for &i in arr.iter() {
    if ans < i {
      break;
    }
    ans += i;
  }

  ans
}
