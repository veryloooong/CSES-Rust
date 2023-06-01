#![allow(dead_code, unused_imports)]
use std::cmp::{min, max, Ordering};
use std::collections::HashMap;
use std::io;

// https://github.com/EbTech/rust-algorithms
struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
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
    let mut scan = Scanner::new(io::stdin().lock());
    let n: usize = scan.token();

    let mut arr: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        let value = scan.token();
        arr.push(value);
    }

    let mut freqs: HashMap<u32, usize>= HashMap::new();

    for x in arr {
        let freq = freqs.entry(x).or_insert(1);
        *freq += 1;
    }

    println!("{}", freqs.len());
}