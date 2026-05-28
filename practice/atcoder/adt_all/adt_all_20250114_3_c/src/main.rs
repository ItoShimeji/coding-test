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
    let _: usize = sc.next();
    let m: usize = sc.next();

    let s: String = sc.next();
    let t: String = sc.next();
    let t_chars = t.as_bytes();

    let mut is_prefix = true;
    for (i, c) in s.as_bytes().iter().enumerate() {
        if *c != t_chars[i] {
            is_prefix = false;
        }
    }

    let mut is_suffix = true;
    for (i, c) in s.as_bytes().iter().rev().enumerate() {
        if *c != t_chars[m - 1 - i] {
            is_suffix = false;
        }
    }

    match (is_prefix, is_suffix) {
        (true, true) => println!("0"),
        (true, false) => println!("1"),
        (false, true) => println!("2"),
        (false, false) => println!("3"),
    };
}
