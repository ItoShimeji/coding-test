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

    let message = if solve(s) { "YES" } else { "NO" };
    println!("{}", message);
}

fn solve(s: String) -> bool {
    let mut start: isize = s.len() as isize;
    let mut end = s.len();

    while end != 0 {
        // println!("{} {}", start, end);
        let str = &s[0..end];
        start -= if str.ends_with("mer") {
            7
        } else if str.ends_with("ser") {
            6
        } else {
            5
        };

        if start < 0 {
            return false;
        }

        match &s[start as usize..end] {
            "dream" => (),
            "dreamer" => (),
            "erase" => (),
            "eraser" => (),
            _ => return false,
        }

        end = start as usize;
    }

    true
}
