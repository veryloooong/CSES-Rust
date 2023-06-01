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

fn main() {
    let mut scan = Scanner::new(stdin().lock());
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = scan.token();
    let x: i64 = scan.token();

    let mut arr: Vec<(i64, usize)> = Vec::with_capacity(n);

    for i in 1..=n {
        let elem: i64 = scan.token();
        arr.push((elem, i));        
    }

    arr.sort_by(|a, b| a.0.cmp(&b.0));

    for (value, index) in &arr {
        match arr.binary_search_by_key(&(x - value), |&(other, _)| other) {
            Err(_) => continue,
            Ok(i) => {
                if index != &arr[i].1 {
                    writeln!(out, "{} {}", index, arr[i].1).ok();
                    return;
                }
            }
        }
    }

    writeln!(out, "IMPOSSIBLE").ok();
}
