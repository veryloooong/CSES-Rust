use std::io::{BufWriter, Write};

fn tower(n: u32, a: u32, b: u32, c: u32, out: &mut BufWriter<std::io::StdoutLock>) {
  if n == 0 {
    return;
  }

  tower(n - 1, a, c, b, out);
  writeln!(out, "{} {}", a, b).ok();
  tower(n - 1, c, b, a, out);
}

fn solve(n: u32, out: &mut BufWriter<std::io::StdoutLock>) {
  writeln!(out, "{}", (2 as u32).pow(n)).ok();

  tower(n, 1, 3, 2, out)
}
