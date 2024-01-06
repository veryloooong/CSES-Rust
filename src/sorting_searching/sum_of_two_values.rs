#![allow(dead_code, unused_imports)]
use std::cmp::{max, min, Ordering};
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

const MOD: u64 = 1_000_000_007;

// https://github.com/EbTech/rust-algorithms
struct Scanner<R> {
  reader: R,
  buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
  fn new(reader: R) -> Self {
    Self {
      reader,
      buffer: vec![],
    }
  }

  fn token<T: std::str::FromStr>(&mut self) -> T {
    loop {
      if let Some(token) = self.buffer.pop() {
        return token.parse().ok().expect("Failed parse");
      }
      let mut input = String::new();
      self.reader.read_line(&mut input).expect("Failed read");
      self.buffer = input.split_whitespace().rev().map(String::from).collect();
    }
  }
}

fn solve(x: i64, arr: &mut [(i64, usize)]) -> Option<(usize, usize)> {
  arr.sort_unstable_by(|a, b| a.0.cmp(&b.0));

  for &(value, index) in arr.iter() {
    if let Ok(i) = arr.binary_search_by_key(&(x - value), |&(other, _)| other) {
      if index != arr[i].1 {
        return Some((index, arr[i].1));
      }
    }
  }

  None
}
