use std::cmp::max;

fn solve(arr: &[i128]) -> i128 {
  let mut ans = i128::MIN;
  let mut cur = 0;

  for &i in arr.iter() {
    cur = max(i, cur + i);
    ans = max(cur, ans);
  }

  ans
}
