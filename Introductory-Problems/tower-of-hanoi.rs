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

fn tower(n: u32, a: u32, b: u32, c: u32, out: &mut BufWriter<std::io::StdoutLock>) {
    if n == 0 {
        return;
    }

    tower(n - 1, a, c, b, out);
    writeln!(out, "{} {}", a, b).ok();
    tower(n - 1, c, b, a, out);
}

fn main() {
    let mut scan = Scanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let n: u32 = scan.token();

    writeln!(out, "{}", (2 as u32).pow(n)).ok();

    tower(n, 1, 3, 2, &mut out)
}
