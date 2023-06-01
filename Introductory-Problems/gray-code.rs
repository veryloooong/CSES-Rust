#![allow(dead_code, unused_imports)]
use std::cmp::{min, max, Ordering};
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

fn print_arr(arr: &Vec<u32>) -> () {
    let mut i = arr.len() - 1;
    while i > 0 && i <= arr.len() {
        print!("{}", arr[i]);
        i -= 1;
    }
    print!("{}\n", arr[0]);
}

fn rightmost_set_bit(num: i32) -> usize {
    return num.trailing_zeros() as usize;
}

fn main() {
    let mut scan = Scanner::new(io::stdin().lock());
    let n: u32 = scan.token();

    let mut arr: Vec<u32> = vec![0; n as usize];

    print_arr(&arr);

    for p in 1..1<<n {
        let lsb = rightmost_set_bit(p);
        arr[lsb] = 1 - arr[lsb];
        print_arr(&arr);
    }
}
