#![allow(dead_code, unused_imports)]
use std::io;

const MAX: usize = 1_000_000_007;

fn input_value<T: std::str::FromStr>() -> T {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read");

    let number: T = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!("Not a number"),
    };

    number
}

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
    let n: u64 = scan.next();

    let mut sum: u64 = 0;

    for _ in 0..n-1 {
        let elem: u64 = scan.next();
        sum += elem;
    }
    
    println!("{}", n * (n + 1) / 2 - sum);
}
