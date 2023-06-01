#![allow(dead_code, unused_imports)]
use std::cmp::{max, min, Ordering};
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

const MOD: i64 = 1_000_000_007;

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

fn main() {
    let mut input = Scanner::new(stdin().lock());
    let mut output = BufWriter::new(stdout().lock());
    
    let n: usize = input.token();

    for _ in 0..n {
        let x: i64 = input.token();
        let mut ans: i64 = 0;

        let mut d = 1;
        while d * d < x {
            if x % d == 0 {
                ans += 2;
            }
            d += 1;
        }
        if d * d == x {
            ans += 1;
        }

        writeln!(output, "{ans}").ok();
    }
}
