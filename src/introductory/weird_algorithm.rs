fn main(mut n: u64) {
  print!("{n} ");

  while n > 1 {
    n = match n % 2 {
      1 => 3 * n + 1,
      0 => n / 2,
      _ => panic!("???"),
    };

    print!("{n} ");
  }
}
