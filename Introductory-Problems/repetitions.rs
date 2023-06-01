#![allow(dead_code, unused_imports)]
use std::{io, cmp};

const MAX: usize = 1_000_000_007;

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
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

fn solve() {
    let mut scan = Scanner::default();
    let str: String = scan.next();
    let n = str.len();
    let mut max: u64 = 1;
    let mut cur: u64 = 1;

    let str = str.into_bytes();

    for i in 1..n {
        if str[i] == str[i - 1] {
            cur += 1;
        } else {
            cur = 1;
        }
        max = cmp::max(max, cur);
    }

    println!("{max}");
}

fn main() {
    // let mut tests = String::new();

    // io::stdin()
    //     .read_line(&mut tests)
    //     .expect("Unknown tests");

    // let tests: u32 = tests.trim().parse().expect("Not a number");

    // for _ in 0..tests {
        solve();
    // }
}
