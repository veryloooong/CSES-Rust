use crate::default_io;

fn solve() -> i32 {
  let (mut input, _) = default_io();

  let n: usize = input.token();
  let mut arr: Vec<usize> = vec![0; n];
  let mut ans = 1;

  for i in 0..n {
    let elem: usize = input.token();
    arr[elem - 1] = i;
  }

  for i in 1..n {
    if arr[i] < arr[i - 1] {
      ans += 1;
    }
  }

  ans
}
