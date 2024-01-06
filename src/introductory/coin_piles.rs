fn solve(mut a: i64, mut b: i64) -> &'static str {
  while a > 0 && b > 0 {
    if a == b {
      let c = a / 3;
      if c == 0 {
        break;
      } else {
        a -= 3 * c;
        b -= 3 * c;
      }
    } else {
      let diff = (a - b).abs();
      if a > b {
        a -= 2 * diff;
        b -= 1 * diff;
      } else {
        a -= 1 * diff;
        b -= 2 * diff;
      }
    }
  }

  if a == 0 && b == 0 {
    "YES"
  } else {
    "NO"
  }
}
