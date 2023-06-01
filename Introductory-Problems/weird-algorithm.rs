#![allow(dead_code, unused_imports)]
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

fn main() {
    let mut scan = Scanner::default();
    let mut n: u64 = scan.next();

    print!("{n} ");

    while n > 1 {
        n = match n % 2 {
            1 => 3 * n + 1,
            0 => n / 2,
            _ => panic!("???"),
        };

        print!("{n} ");
    }
}
