fn solve(n: usize) {
  match n {
    1 => println!("1"),
    2 | 3 => println!("NO SOLUTION"),
    _ => {
      let mut i = 2;
      while i <= n {
        print!("{i} ");
        i += 2;
      }
      i = 1;
      while i <= n {
        print!("{i} ");
        i += 2;
      }
    }
  }
}
