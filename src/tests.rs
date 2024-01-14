#[cfg(test)]
mod tests {
  use crate::gcd;

  #[test]
  fn gcd_test() {
    assert_eq!(gcd(3, 5), 1);
    assert_eq!(gcd(12, 18), 6);

    let p = i8::default();
    assert_eq!(p, 0);

    let n = 13i32.trailing_zeros();
  }
}
