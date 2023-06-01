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
    
    let mut arr: Vec<i64> = Vec::with_capacity(n + 1);
    
    arr.push(1);

    for _ in 0..n {
        arr.push(0);
    }

    for a in 1..=n {
        for b in 1..=min(6, a) {
            arr[a] = (arr[a] + arr[a - b]) % MOD;
        }
    }

    writeln!(output, "{}", arr[n]).ok();
}
