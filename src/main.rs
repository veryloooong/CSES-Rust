#![allow(dead_code)]

use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, StdinLock, StdoutLock, Write};

mod dynamic_programming;
mod introductory;
mod mathematics;
mod sorting_searching;

#[inline]
fn gcd<T>(a: T, b: T) -> T
where
  T: PartialEq + std::ops::Rem<Output = T> + Default + Copy,
{
  if b == T::default() {
    a
  } else {
    gcd(b, a % b)
  }
}

#[inline]
fn lcm<T>(a: T, b: T) -> T
where
  T: PartialEq
    + std::ops::Rem<Output = T>
    + std::ops::Mul<Output = T>
    + std::ops::Div<Output = T>
    + Default
    + Copy,
{
  a * b / gcd(a, b)
}

/// https://github.com/EbTech/rust-algorithms
struct Scanner<R> {
  reader: R,
  buffer: Vec<String>,
}

impl<R: std::io::BufRead> Scanner<R> {
  fn new(reader: R) -> Self {
    Self {
      reader,
      buffer: vec![],
    }
  }

  pub(crate) fn token<T: std::str::FromStr>(&mut self) -> T {
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

/// Quick for initing IO
fn default_io() -> (Scanner<StdinLock<'static>>, BufWriter<StdoutLock<'static>>) {
  (
    Scanner::new(stdin().lock()),
    BufWriter::new(stdout().lock()),
  )
}

/// For local tests
fn from_file() -> (Scanner<BufReader<File>>, BufWriter<File>) {
  let input = File::open("./src/input.txt").expect("Not found");
  let output = File::create("./src/output.txt").expect("Not found");

  (
    Scanner::new(std::io::BufReader::new(input)),
    std::io::BufWriter::new(output),
  )
}

fn main() -> std::io::Result<()> {
  let (mut input, mut output) = default_io();

  Ok(())
}

/* uuuuuuuuuuuuuuu - Ceres Fauna (2021 - ) */
