use std::io::{self, Read};

struct Scanner {
    input: Vec<u8>,
    index: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();

        Scanner { input, index: 0 }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        while self.index < self.input.len() && self.input[self.index].is_ascii_whitespace() {
            self.index += 1;
        }

        let start = self.index;
        while self.index < self.input.len() && !self.input[self.index].is_ascii_whitespace() {
            self.index += 1;
        }

        std::str::from_utf8(&self.input[start..self.index])
            .unwrap()
            .parse::<T>()
            .ok()
            .unwrap()
    }
}

fn main() {
    let mut sc = Scanner::new();
    let s: String = sc.next();

    let message = if solve(&s) { "YES" } else { "NO" };
    println!("{}", message);
}

fn solve(mut s: &str) -> bool {
    while !s.is_empty() {
        if let Some(rest) = s.strip_suffix("dream") {
            s = rest;
        } else if let Some(rest) = s.strip_suffix("dreamer") {
            s = rest;
        } else if let Some(rest) = s.strip_suffix("erase") {
            s = rest;
        } else if let Some(rest) = s.strip_suffix("eraser") {
            s = rest;
        } else {
            return false;
        }
    }

    true
}
