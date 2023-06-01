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
    let n: usize = scan.next();
    let mut arr: Vec<i64> = Vec::with_capacity(n);

    for _ in 0..n {
        let elem: i64 = scan.next();
        arr.push(elem);
    }

    let mut ans: i64 = 0;

    for i in 1..n {
        let diff: i64 = arr[i] - arr[i - 1];
        match diff.cmp(&0) {
            Ordering::Less => {
                ans -= diff;
                arr[i] = arr[i - 1];
            },
            Ordering::Equal | Ordering::Greater => continue,
        }
    }
    
    println!("{ans}");
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
