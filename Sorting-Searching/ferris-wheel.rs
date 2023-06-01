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

fn main() {
    let mut scan = Scanner::default();
    let n: usize = scan.next();
    let x: u64 = scan.next();

    let mut arr: Vec<u64> = Vec::with_capacity(n);
    for _ in 0..n {
        let elem = scan.next();
        arr.push(elem);
    }

    arr.sort_unstable();

    let mut lhs: usize = 0;
    let mut rhs: usize = n - 1;
    let mut ans = 0;

    while lhs <= rhs && rhs < n {
        if arr[lhs] + arr[rhs] <= x {
            lhs += 1;
        }
        ans += 1;
        rhs -= 1;
    }

    println!("{ans}");
}
