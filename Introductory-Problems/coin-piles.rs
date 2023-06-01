#![allow(dead_code, unused_imports)]
use std::cmp::{min, max, Ordering};
use std::io;

const MOD: usize = 1_000_000_007;

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

fn solve() -> () {
    let mut scan = Scanner::default();

    let mut a: i64 = scan.next();
    let mut b: i64 = scan.next();

    while a > 0 && b > 0 {
        if a == b {
            let c = a / 3;
            if c == 0 {
                break;
            } else {
                a -= 3 * c;
                b -= 3 * c;
            }
        } else {
            let diff = (a - b).abs();
            if a > b {
                a -= 2 * diff;
                b -= 1 * diff;
            } else {
                a -= 1 * diff;
                b -= 2 * diff;
            }
        }
    }

    if a == 0 && b == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn main() {
    let mut tests = String::new();

    io::stdin()
        .read_line(&mut tests)
        .expect("Unknown tests");

    let tests: u32 = tests.trim().parse().expect("Not a number");

    for _ in 0..tests {
        solve();
    }
}
