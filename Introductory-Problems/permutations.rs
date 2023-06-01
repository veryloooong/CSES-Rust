#![allow(dead_code, unused_imports)]
use std::cmp::{min, max, Ordering};
use std::io;

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

fn solve() -> () {
    let mut scan = Scanner::default();
    let n: u32 = scan.next();

    match n {
        1 => println!("1"),
        2 | 3 => println!("NO SOLUTION"),
        _ => {
            let mut i: u32 = 2;
            while i <= n {
                print!("{i} ");
                i += 2;
            }
            i = 1;
            while i <= n {
                print!("{i} ");
                i += 2;
            }
        }
    }
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
