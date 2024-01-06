use std::cmp::min;

fn solve(arr: &[usize]) -> usize {
  let mut s1;
  let mut s2;
  let mut ans = usize::MAX;

  let n = arr.len();

  for i in 0..(1 << n) {
    s1 = 0;
    s2 = 0;
    for j in 0..n {
      if (1usize << j) & i == 1 {
        s1 += arr[j];
      } else {
        s2 += arr[j];
      }
    }

    ans = min(ans, s1.abs_diff(s2));
  }

  ans
}
