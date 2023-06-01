#![allow(dead_code, unused_imports)]
use std::cmp::{min, max, Ordering};
use std::io;

const MOD: u64 = 1_000_000_007;

// https://github.com/EbTech/rust-algorithms
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Unable to parse");
            }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Unable to read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let n: u64 = scan.token();
    let mut ans: u64 = 0;

    let mut i: u64 = 5;

    while n / i >= 1 {
        ans += n / i;
        i *= 5;
    }

    println!("{ans}");
}
