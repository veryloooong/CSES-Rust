fn solve(n: usize) -> Option<(Vec<usize>, Vec<usize>)> {
  let s = n * (n + 1) / 2;
  if s % 2 == 1 {
    None
  } else {
    let mut s = s / 2;
    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    for i in (1..=n).rev() {
      if s >= i {
        arr1.push(i);
        s -= i;
      } else {
        arr2.push(i);
      }
    }

    Some((arr1, arr2))
  }
}
